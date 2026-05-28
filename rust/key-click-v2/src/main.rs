use std::{sync::mpsc, thread};

mod config;
mod input_device;
mod keyboard;
mod overlay;

fn main() {
    // Force GTK to use XWayland so fullscreen RGBA windows composite correctly.
    // GNOME Wayland's native path uses direct scanout for fullscreen windows
    // which bypasses alpha blending entirely.
    unsafe { std::env::set_var("GDK_BACKEND", "x11") };

    let config = std::sync::Arc::new(config::Config::load());

    let (tx, rx) = mpsc::channel::<keyboard::Message>();

    let config_kb = std::sync::Arc::clone(&config);
    thread::spawn(move || {
        keyboard::start_listener(tx, &config_kb);
    });

    overlay::run(rx, &config);
}
