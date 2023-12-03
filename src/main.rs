/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use evdev::{Device, EventType, InputEventKind, Key};
use rodio::{OutputStream, Sink};
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
    let mut input_device = None;

    for (path, device) in evdev::enumerate() {
        println!(
            "Path: {} - Unique Name: {:?}",
            path.display(),
            device.unique_name()
        );
        if device.unique_name() == Some("08FF20140315") {
            input_device = Some(path);
            break;
        } else {
            panic!("No RFID reader connected!");
        }
    }

    let config_filename = PathBuf::from("/home/pablo/.config/soundboard/config.toml");

    let config = config::load_config(&config_filename)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut input_device = Device::open(&args.input_device)?;
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
            // Only handle pressed key events.
            if event.event_type() != EventType::KEY || event.value() == 1 {
                continue;
            }

            match event.kind() {
                InputEventKind::Key(key) => {
                    if let Some(ch) = get_char(key) {
                        read_chars.push(ch)
                    }
                    println!("Read chars: {}", read_chars);
                    if read_chars.len() == 10 {
                        let input = read_chars.as_str();
                        audio::play_sound(
                            &config.inputs_to_filenames,
                            input,
                            config.sounds_path.as_path(),
                            &sink,
                        )?;
                        read_chars.clear();
                    }
                }
                _ => (),
            }
        }
    }
}
