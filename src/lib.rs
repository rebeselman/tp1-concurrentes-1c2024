//! Hi! :), this is my implementation of the tp1
use config::Config;
use line::Line;
use serde_json::{self, json};
use site::Site;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::fs::read_dir;
use std::io::BufRead;
use std::io::BufReader;
pub mod config;
pub mod line;
pub mod site;
pub mod tag;
pub mod totals;


/// Function which runs the application
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    // Command::new("/bin/sh").arg("download_data.sh").output()?;
    // let result = read_dir("/data1").unwrap()
    // .map(|d| d.unwrap().path())
    // .flat_map(|path| {
    //     let file = File::open(path);
    //     let reader = BufReader::new(file.unwrap());
    //     reader.lines()
    // })
    // .map(|l| {
    //     let l = l?;
    //     let line: Line = serde_json::from_str(&l)?;
    //     let mut counts = HashMap::new();
    //     words.for_each(|w| *counts.entry(w.to_string()).or_insert(0) += 1);
    //     counts
    // }).fold(init, f);

    Ok(())
}
