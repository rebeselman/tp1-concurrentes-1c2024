//! Tp1
use config::Config;
use serde_json::{self, json};
use std::error::Error;

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use crate::process::process_files_in_parallel;
use crate::totals::Totals;

pub mod process;
pub mod utilities;

pub mod config;
pub mod question;
pub mod site;
pub mod tag;
pub mod totals;

/// Function which runs the application
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    Command::new("/bin/sh").arg("download_data.sh").output()?;

    let file_paths: Vec<PathBuf> = fs::read_dir("data")?
        .map(|entry| match entry {
            Ok(entry) => entry.path(),
            Err(_) => PathBuf::new(),
        })
        .filter(|path| {
            if let Some(e) = path.extension() {
                e == "jsonl"
            } else {
                false
            }
        })
        .collect();

    // Process
    let start = Instant::now();
    let result = process_files_in_parallel(file_paths, c.number_of_threads)?;

    let totals = Totals::new_from(&result.1, &result.0);
    println!("tiempo para procesar todo: {:?}", start.elapsed());

    // Create structura JSON
    let json_data = json!({
        "padron": 108672,
        "sites": result.0,
        "tags": result.1,
        "totals": totals

    });
    let formatted_json = serde_json::to_string_pretty(&json_data)?;
    println!("{}", formatted_json);
    Ok(())
}
