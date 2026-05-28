use std::sync::mpsc::Sender;

use evdev::{Device, EventType};

use crate::config::Config;
use crate::input_device::find_keyboard;

#[derive(Clone, Copy)]
pub enum Message {
    ShowGrid,
}

const J_KEY_CODE: u16 = 36;
const LEFT_SHIFT_KEY_CODE: u16 = 42;
const RIGHT_SHIFT_KEY_CODE: u16 = 54;
const LEFT_CTRL_KEY_CODE: u16 = 29;
const RIGHT_CTRL_KEY_CODE: u16 = 97;
const LEFT_ALT_KEY_CODE: u16 = 56;
const RIGHT_ALT_KEY_CODE: u16 = 100;
const KEY_PRESS: i32 = 1;
const KEY_RELEASE: i32 = 0;

pub fn start_listener(tx: Sender<Message>, config: &Config) {
    let mut device = loop {
        match find_keyboard(config.keyboard_path.as_deref()) {
            Ok(path) => match Device::open(path) {
                Ok(d) => break d,
                Err(e) => eprintln!("failed to open keyboard: {e} — retrying in 1s"),
            },
            Err(_) => eprintln!("no keyboard found — retrying in 1s"),
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    };

    let mut ctrl_held = false;
    let mut shift_held = false;
    let mut alt_held = false;

    loop {
        let mut device_lost = false;

        {
            match device.fetch_events() {
                Err(e) => {
                    eprintln!("keyboard device error: {e} — retrying in 1s");
                    device_lost = true;
                }
                Ok(events) => {
                    for event in events {
                        if event.event_type() != EventType::KEY {
                            continue;
                        }
                        match event.value() {
                            KEY_PRESS => match event.code() {
                                LEFT_CTRL_KEY_CODE | RIGHT_CTRL_KEY_CODE => ctrl_held = true,
                                LEFT_SHIFT_KEY_CODE | RIGHT_SHIFT_KEY_CODE => shift_held = true,
                                LEFT_ALT_KEY_CODE | RIGHT_ALT_KEY_CODE => alt_held = true,
                                J_KEY_CODE if ctrl_held && shift_held && alt_held => {
                                    let _ = tx.send(Message::ShowGrid);
                                }
                                _ => {}
                            },
                            KEY_RELEASE => match event.code() {
                                LEFT_CTRL_KEY_CODE | RIGHT_CTRL_KEY_CODE => ctrl_held = false,
                                LEFT_SHIFT_KEY_CODE | RIGHT_SHIFT_KEY_CODE => shift_held = false,
                                LEFT_ALT_KEY_CODE | RIGHT_ALT_KEY_CODE => alt_held = false,
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
            }
        }

        if device_lost {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                match find_keyboard(config.keyboard_path.as_deref()) {
                    Ok(path) => match Device::open(path) {
                        Ok(d) => {
                            eprintln!("keyboard reconnected");
                            device = d;
                            break;
                        }
                        Err(_) => eprintln!("waiting for keyboard..."),
                    },
                    Err(_) => eprintln!("waiting for keyboard..."),
                }
            }
        }
    }
}
