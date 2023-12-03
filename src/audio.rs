/*
 * Copyright 2021-2022 Jochen Kupperschmidt
 * License: MIT
 */

use anyhow::Result;
use rodio::{Decoder, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub(crate) fn play_sound(
    inputs_to_filenames: &HashMap<String, String>,
    input: &str,
    dir: &Path,
    sink: &Sink,
) -> Result<()> {
    if let Some(filename) = inputs_to_filenames.get(input.trim()) {
        let path = dir.join(filename);
        if !&path.exists() {
            eprintln!("Sound file {} does not exist.", path.display());
            return Ok(());
        }
        Command::new("mpg321")
            .arg("-o alsa")
            .arg(path)
            .spawn()?
            .wait()?;
    }
    Ok(())
}

fn load_source(path: &Path) -> Result<Decoder<BufReader<File>>> {
    let file = BufReader::new(File::open(path)?);
    Ok(Decoder::new(file)?)
}
