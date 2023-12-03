/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use execute::Execute;
use std::process::Command;

use crate::config::Config;

pub(crate) fn play_sound(config: &Config, identifier: &str) -> Result<()> {
    if let Some(filename) = config.inputs_to_filenames.get(identifier.trim()) {
        let path = config.sounds_path.join(filename);
        if !&path.exists() {
            eprintln!("Sound file {} does not exist.", path.display());
            return Ok(());
        }
        let mut command = Command::new("mpg123");
        command.arg(path);

        if let Some(exit_code) = command.execute().unwrap() {
            if exit_code == 0 {
                println!("Ok.");
            } else {
                eprintln!("Failed.");
            }
        } else {
            eprintln!("Interrupted!");
        }
    }
    Ok(())
}
