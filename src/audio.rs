/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use execute::Execute;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

pub(crate) fn play_sound(
    inputs_to_filenames: &HashMap<String, String>,
    input: &str,
    dir: &Path,
) -> Result<()> {
    if let Some(filename) = inputs_to_filenames.get(input.trim()) {
        let path = dir.join(filename);
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
