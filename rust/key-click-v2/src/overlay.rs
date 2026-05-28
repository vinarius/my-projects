use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use gtk::prelude::*;
use gtk::{cairo, glib, Application, ApplicationWindow, DrawingArea, EventControllerKey};
use x11rb::connection::Connection;
use x11rb::protocol::xtest::ConnectionExt as XTestExt;
use x11rb::rust_connection::RustConnection;

use crate::config::Config;
use crate::keyboard::Message;

#[derive(Clone, Copy, PartialEq)]
enum OverlayMode {
    Normal,
    Zoom(u8, u8),
}

#[derive(Clone, Copy)]
enum GridState {
    Idle,
    ColSelected(u8),
    ZoomActive(u8, u8),
}

fn cols_per_monitor(grid_size: i32, total_monitors: usize) -> i32 {
    grid_size / total_monitors as i32
}

fn key_to_idx(key: gtk::gdk::Key) -> Option<u8> {
    let c = key.to_unicode()?.to_ascii_lowercase();
    if c >= 'a' && c <= 'z' {
        Some((c as u8) - b'a')
    } else {
        None
    }
}

fn make_draw_func(
    monitor_idx: usize,
    total_monitors: usize,
    grid_size: i32,
    zoom_size: i32,
    overlay_opacity: f64,
    highlight_col: Rc<Cell<Option<u8>>>,
    highlight_cell: Rc<Cell<Option<(u8, u8)>>>,
    overlay_mode: Rc<Cell<OverlayMode>>,
) -> impl Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static {
    move |_, cr, width, height| {
        cr.set_operator(cairo::Operator::Clear);
        cr.paint().unwrap();
        cr.set_operator(cairo::Operator::Over);

        let cpm = cols_per_monitor(grid_size, total_monitors);
        let col_start = monitor_idx as i32 * cpm;
        let col_end = if monitor_idx == total_monitors - 1 { grid_size } else { col_start + cpm };
        let local_cols = col_end - col_start;
        let cell_w = width as f64 / local_cols as f64;
        let cell_h = height as f64 / grid_size as f64;

        draw_normal(cr, width, height, monitor_idx, total_monitors, grid_size, overlay_opacity, highlight_col.get(), highlight_cell.get());

        if let OverlayMode::Zoom(cell_col, cell_row) = overlay_mode.get() {
            let local_col = cell_col as i32 - col_start;
            if local_col >= 0 && local_col < local_cols {
                let ox = local_col as f64 * cell_w;
                let oy = cell_row as f64 * cell_h;
                draw_zoom(cr, ox, oy, cell_w, cell_h, zoom_size, highlight_cell.get());
            }
        }
    }
}

fn draw_normal(
    cr: &cairo::Context,
    width: i32, height: i32,
    monitor_idx: usize, total_monitors: usize,
    grid_size: i32,
    overlay_opacity: f64,
    highlight_col: Option<u8>,
    highlight_cell: Option<(u8, u8)>,
) {
    let cpm = cols_per_monitor(grid_size, total_monitors);
    let col_start = monitor_idx as i32 * cpm;
    let col_end = if monitor_idx == total_monitors - 1 { grid_size } else { col_start + cpm };
    let local_cols = col_end - col_start;
    let cell_w = width as f64 / local_cols as f64;
    let cell_h = height as f64 / grid_size as f64;

    cr.set_source_rgba(0.0, 0.0, 0.0, overlay_opacity);
    cr.paint().unwrap();

    if let Some(hcol) = highlight_col {
        let local_col = hcol as i32 - col_start;
        if local_col >= 0 && local_col < local_cols {
            cr.set_source_rgba(1.0, 1.0, 0.0, 0.15);
            cr.rectangle(local_col as f64 * cell_w, 0.0, cell_w, height as f64);
            cr.fill().unwrap();
        }
    }

    if let Some((hcol, hrow)) = highlight_cell {
        let local_col = hcol as i32 - col_start;
        if local_col >= 0 && local_col < local_cols {
            cr.set_source_rgba(1.0, 1.0, 0.0, 0.5);
            cr.rectangle(local_col as f64 * cell_w, hrow as f64 * cell_h, cell_w, cell_h);
            cr.fill().unwrap();
        }
    }

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.15);
    cr.set_line_width(0.5);
    for col in 0..local_cols {
        for row in 0..grid_size {
            cr.rectangle(col as f64 * cell_w + 0.5, row as f64 * cell_h + 0.5, cell_w, cell_h);
            cr.stroke().unwrap();
        }
    }

    cr.select_font_face("monospace", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.set_font_size(16.0);

    for local_col in 0..local_cols {
        let global_col = col_start + local_col;
        for row in 0..grid_size {
            let x = local_col as f64 * cell_w;
            let y = row as f64 * cell_h;
            let label = format!(
                "{}{}",
                (b'a' + global_col as u8) as char,
                (b'a' + row as u8) as char,
            );
            let ext = cr.text_extents(&label).unwrap();
            let tx = x + (cell_w - ext.width()) / 2.0 - ext.x_bearing();
            let ty = y + (cell_h - ext.height()) / 2.0 - ext.y_bearing();
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.8);
            cr.move_to(tx + 1.0, ty + 1.0);
            cr.show_text(&label).unwrap();
            cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            cr.move_to(tx, ty);
            cr.show_text(&label).unwrap();
        }
    }
}

fn draw_zoom(
    cr: &cairo::Context,
    ox: f64, oy: f64,
    ow: f64, oh: f64,
    zoom_size: i32,
    highlight_cell: Option<(u8, u8)>,
) {
    let sub_w = ow / zoom_size as f64;
    let sub_h = oh / zoom_size as f64;

    cr.set_source_rgba(0.05, 0.05, 0.15, 0.92);
    cr.rectangle(ox, oy, ow, oh);
    cr.fill().unwrap();

    if let Some((sc, sr)) = highlight_cell {
        cr.set_source_rgba(1.0, 1.0, 0.0, 0.6);
        cr.rectangle(ox + sc as f64 * sub_w, oy + sr as f64 * sub_h, sub_w, sub_h);
        cr.fill().unwrap();
    }

    cr.set_source_rgba(1.0, 1.0, 1.0, 0.25);
    cr.set_line_width(0.5);
    for col in 0..zoom_size {
        for row in 0..zoom_size {
            cr.rectangle(ox + col as f64 * sub_w + 0.5, oy + row as f64 * sub_h + 0.5, sub_w, sub_h);
            cr.stroke().unwrap();
        }
    }

    let font_size = (sub_h * 0.35).max(6.0);
    cr.select_font_face("monospace", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
    cr.set_font_size(font_size);

    for sub_col in 0..zoom_size {
        for sub_row in 0..zoom_size {
            let idx = sub_row * zoom_size + sub_col;
            let label = format!("{}", (b'a' + idx as u8) as char);
            let x = ox + sub_col as f64 * sub_w;
            let y = oy + sub_row as f64 * sub_h;
            let ext = cr.text_extents(&label).unwrap();
            let tx = x + (sub_w - ext.width()) / 2.0 - ext.x_bearing();
            let ty = y + (sub_h - ext.height()) / 2.0 - ext.y_bearing();
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.8);
            cr.move_to(tx + 1.0, ty + 1.0);
            cr.show_text(&label).unwrap();
            cr.set_source_rgba(1.0, 1.0, 0.3, 1.0);
            cr.move_to(tx, ty);
            cr.show_text(&label).unwrap();
        }
    }

    cr.set_source_rgba(1.0, 1.0, 0.0, 0.8);
    cr.set_line_width(2.0);
    cr.rectangle(ox + 1.0, oy + 1.0, ow - 2.0, oh - 2.0);
    cr.stroke().unwrap();
}

fn compute_fine_position(
    geo: &gtk::gdk::Rectangle,
    grid_size: i32,
    zoom_size: i32,
    local_cols: i32,
    local_col: i32,
    row: u8,
    sub_col: u8,
    sub_row: u8,
) -> (i32, i32) {
    let cell_w = geo.width() as f64 / local_cols as f64;
    let cell_h = geo.height() as f64 / grid_size as f64;
    let cell_x = local_col as f64 * cell_w + geo.x() as f64;
    let cell_y = row as f64 * cell_h + geo.y() as f64;
    let sub_w = cell_w / zoom_size as f64;
    let sub_h = cell_h / zoom_size as f64;
    let x = cell_x + (sub_col as f64 + 0.5) * sub_w;
    let y = cell_y + (sub_row as f64 + 0.5) * sub_h;
    (x as i32, y as i32)
}

fn build_windows(
    app: &Application,
    monitor_list: &[gtk::gdk::Monitor],
    total_monitors: usize,
    grid_size: i32,
    zoom_size: i32,
    overlay_opacity: f64,
    highlight_col: &Rc<Cell<Option<u8>>>,
    highlight_cell: &Rc<Cell<Option<(u8, u8)>>>,
    overlay_mode: &Rc<Cell<OverlayMode>>,
) -> Vec<(ApplicationWindow, DrawingArea, gtk::gdk::Monitor)> {
    let css = gtk::CssProvider::new();
    css.load_from_data("window { background-color: transparent; }");
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    monitor_list.iter().enumerate().map(|(idx, monitor)| {
        let window = ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .build();

        window.connect_realize(|w| {
            if let Some(surface) = w.surface() {
                surface.set_opaque_region(None);
            }
        });

        window.fullscreen_on_monitor(monitor);

        let drawing_area = DrawingArea::new();
        drawing_area.set_draw_func(make_draw_func(
            idx,
            total_monitors,
            grid_size,
            zoom_size,
            overlay_opacity,
            Rc::clone(highlight_col),
            Rc::clone(highlight_cell),
            Rc::clone(overlay_mode),
        ));
        window.set_child(Some(&drawing_area));
        (window, drawing_area, monitor.clone())
    }).collect()
}

pub fn run(rx: Receiver<Message>, config: &Config) {
    let grid_size = config.grid_size as i32;
    let zoom_size = config.zoom_size as i32;
    let overlay_opacity = config.overlay_opacity;

    let app = Application::builder().build();

    // prevent GTK from quitting when all windows are closed
    let _hold = app.hold();

    let rx = Rc::new(RefCell::new(rx));

    app.connect_activate(move |app| {
        let display = gtk::gdk::Display::default().unwrap();
        let monitors = display.monitors();

        let monitor_list: Vec<gtk::gdk::Monitor> = (0..monitors.n_items())
            .filter_map(|i| {
                monitors
                    .item(i)
                    .and_then(|m| m.downcast::<gtk::gdk::Monitor>().ok())
            })
            .collect();

        let total_monitors = monitor_list.len();

        let monitor_geometries: Rc<Vec<gtk::gdk::Rectangle>> =
            Rc::new(monitor_list.iter().map(|m| m.geometry()).collect());

        let highlight_col: Rc<Cell<Option<u8>>> = Rc::new(Cell::new(None));
        let highlight_cell: Rc<Cell<Option<(u8, u8)>>> = Rc::new(Cell::new(None));
        let overlay_mode: Rc<Cell<OverlayMode>> = Rc::new(Cell::new(OverlayMode::Normal));
        let grid_state: Rc<Cell<GridState>> = Rc::new(Cell::new(GridState::Idle));

        let (xconn, _) = RustConnection::connect(None).expect("failed to connect to X11 for mouse control");
        let xconn: Rc<RustConnection> = Rc::new(xconn);

        // active windows — empty until ShowGrid
        let active_windows: Rc<RefCell<Vec<(ApplicationWindow, DrawingArea, gtk::gdk::Monitor)>>> =
            Rc::new(RefCell::new(Vec::new()));

        let app_ref = app.clone();
        let monitor_list = Rc::new(monitor_list);

        let make_key_handler = {
            let active_windows = Rc::clone(&active_windows);
            let highlight_col = Rc::clone(&highlight_col);
            let highlight_cell = Rc::clone(&highlight_cell);
            let overlay_mode = Rc::clone(&overlay_mode);
            let grid_state = Rc::clone(&grid_state);
            let monitor_geometries = Rc::clone(&monitor_geometries);
            let xconn = Rc::clone(&xconn);

            move || {
                let key_ctrl = EventControllerKey::new();
                let key_ctrl_ref = key_ctrl.clone();
                let active_windows = Rc::clone(&active_windows);
                let highlight_col = Rc::clone(&highlight_col);
                let highlight_cell = Rc::clone(&highlight_cell);
                let overlay_mode = Rc::clone(&overlay_mode);
                let grid_state = Rc::clone(&grid_state);
                let monitor_geometries = Rc::clone(&monitor_geometries);
                let xconn = Rc::clone(&xconn);

                key_ctrl.connect_key_pressed(move |_, key, _, _| {
                    let dismiss = || {
                        highlight_col.set(None);
                        highlight_cell.set(None);
                        overlay_mode.set(OverlayMode::Normal);
                        grid_state.set(GridState::Idle);
                        for (w, _, _) in active_windows.borrow().iter() {
                            w.close();
                        }
                        active_windows.borrow_mut().clear();
                    };

                    let redraw_all = || {
                        for (_, da, _) in active_windows.borrow().iter() {
                            da.queue_draw();
                        }
                    };

                    if key == gtk::gdk::Key::Escape {
                        let active_windows = Rc::clone(&active_windows);
                        let hcol = Rc::clone(&highlight_col);
                        let hcell = Rc::clone(&highlight_cell);
                        let omode = Rc::clone(&overlay_mode);
                        let gstate = Rc::clone(&grid_state);
                        glib::idle_add_local_once(move || {
                            hcol.set(None);
                            hcell.set(None);
                            omode.set(OverlayMode::Normal);
                            gstate.set(GridState::Idle);
                            for (w, _, _) in active_windows.borrow().iter() {
                                w.close();
                            }
                            active_windows.borrow_mut().clear();
                        });
                        return glib::Propagation::Stop;
                    }

                    let idx = match key_to_idx(key) {
                        Some(i) => i,
                        None => return glib::Propagation::Stop,
                    };

                    match grid_state.get() {
                        GridState::ColSelected(col) => {
                            let row = idx;
                            let mods = key_ctrl_ref.current_event_state();
                            if mods.contains(gtk::gdk::ModifierType::SHIFT_MASK) {
                                grid_state.set(GridState::ZoomActive(col, row));
                                overlay_mode.set(OverlayMode::Zoom(col, row));
                                highlight_col.set(None);
                                highlight_cell.set(None);
                                redraw_all();
                            } else {
                                let cpm = cols_per_monitor(grid_size, total_monitors);
                                let monitor_idx = ((col as i32) / cpm).min(total_monitors as i32 - 1) as usize;
                                let col_start = monitor_idx as i32 * cpm;
                                let col_end = if monitor_idx == total_monitors - 1 { grid_size } else { col_start + cpm };
                                let local_cols = col_end - col_start;
                                let local_col = col as i32 - col_start;
                                let geo = monitor_geometries.get(monitor_idx).or_else(|| monitor_geometries.first()).unwrap();
                                let cell_w = geo.width() as f64 / local_cols as f64;
                                let cell_h = geo.height() as f64 / grid_size as f64;
                                let x = (local_col as f64 + 0.5) * cell_w + geo.x() as f64;
                                let y = (row as f64 + 0.5) * cell_h + geo.y() as f64;
                                let _ = xconn.xtest_fake_input(6, 0, 0, x11rb::NONE, x as i16, y as i16, 0);
                                let _ = xconn.flush();
                                dismiss();
                            }
                        }
                        GridState::ZoomActive(cell_col, cell_row) => {
                            if (idx as i32) < zoom_size * zoom_size {
                                let sub_col = idx % zoom_size as u8;
                                let sub_row = idx / zoom_size as u8;
                                let cpm = cols_per_monitor(grid_size, total_monitors);
                                let monitor_idx = ((cell_col as i32) / cpm).min(total_monitors as i32 - 1) as usize;
                                let col_start = monitor_idx as i32 * cpm;
                                let col_end = if monitor_idx == total_monitors - 1 { grid_size } else { col_start + cpm };
                                let local_cols = col_end - col_start;
                                let local_col = cell_col as i32 - col_start;
                                let geo = monitor_geometries.get(monitor_idx).or_else(|| monitor_geometries.first()).unwrap();
                                let (x, y) = compute_fine_position(geo, grid_size, zoom_size, local_cols, local_col, cell_row, sub_col, sub_row);
                                let _ = xconn.xtest_fake_input(6, 0, 0, x11rb::NONE, x as i16, y as i16, 0);
                                let _ = xconn.flush();
                                dismiss();
                            }
                        }
                        GridState::Idle => {
                            if (idx as i32) < grid_size {
                                grid_state.set(GridState::ColSelected(idx));
                                highlight_col.set(Some(idx));
                                highlight_cell.set(None);
                                redraw_all();
                            }
                        }
                    }

                    glib::Propagation::Stop
                });

                key_ctrl
            }
        };

        let rx: Rc<RefCell<Receiver<Message>>> = Rc::clone(&rx);
        let grid_state_ref = Rc::clone(&grid_state);

        glib::timeout_add_local(Duration::from_millis(50), move || {
            if let Ok(Message::ShowGrid) = rx.borrow_mut().try_recv() {
                highlight_col.set(None);
                highlight_cell.set(None);
                overlay_mode.set(OverlayMode::Normal);
                grid_state_ref.set(GridState::Idle);

                // close any existing windows first
                for (w, _, _) in active_windows.borrow().iter() {
                    w.close();
                }

                // create fresh windows for this activation
                let windows = build_windows(
                    &app_ref,
                    &monitor_list,
                    total_monitors,
                    grid_size,
                    zoom_size,
                    overlay_opacity,
                    &highlight_col,
                    &highlight_cell,
                    &overlay_mode,
                );

                for (window, _, _) in &windows {
                    window.add_controller(make_key_handler());
                    window.present();
                }

                *active_windows.borrow_mut() = windows;
            }
            glib::ControlFlow::Continue
        });
    });

    app.run();
}
