/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use evdev::{EventType, InputEventKind, Key};
use std::path::PathBuf;
use std::process::exit;
mod audio;
mod config;

fn get_char(key: Key) -> Option<char> {
    match key {
        Key::KEY_1 => Some('1'),
        Key::KEY_2 => Some('2'),
        Key::KEY_3 => Some('3'),
        Key::KEY_4 => Some('4'),
        Key::KEY_5 => Some('5'),
        Key::KEY_6 => Some('6'),
        Key::KEY_7 => Some('7'),
        Key::KEY_8 => Some('8'),
        Key::KEY_9 => Some('9'),
        Key::KEY_0 => Some('0'),
        _ => None,
    }
}

fn main() -> Result<()> {
    let config_file = PathBuf::from(format!(
        "{}/.config/soundboard/config.toml",
        std::env::var("HOME").unwrap()
    ));

    let config = config::load_config(&config_file)?;

    let mut input_device = evdev::enumerate()
        .find(|(_, device)| device.unique_name() == Some(&config.rfid_unique_name))
        .unwrap_or_else(|| {
            panic!(
                "RFID reader with unique name {} not found",
                config.rfid_unique_name
            )
        })
        .1;

    println!(
        "Opened input device \"{}\".",
        input_device.name().unwrap_or("unnamed device")
    );

    match input_device.grab() {
        Ok(_) => println!("Successfully obtained exclusive access to input device."),
        Err(error) => {
            eprintln!("Could not get exclusive access to input device: {}", error);
            exit(1);
        }
    }

    let mut read_chars = String::new();
    loop {
        for event in input_device.fetch_events()? {
            // Only released key events.
            if event.event_type() != EventType::KEY || event.value() == 1 {
                continue;
            }

            if let InputEventKind::Key(key) = event.kind() {
                if let Some(ch) = get_char(key) {
                    read_chars.push(ch)
                }
                if read_chars.len() == 10 {
                    let identifier = read_chars.as_str();
                    audio::play_sound(&config, identifier)?;
                    read_chars.clear();
                }
            }
        }
    }
}
