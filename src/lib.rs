//! Hi! :), this is my implementation of the tp1
use config::Config;
use question::Question;
use serde_json::{self, json};
use site::Site;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
pub mod config;
pub mod question;
pub mod site;
pub mod tag;
pub mod totals;

/// Function which runs the application
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    //Command::new("/bin/sh").arg("download_data.sh").output()?;

    // forma con iteradores
    // let sites = read_dir("data1")
    //     .expect("Error al leer directorio")
    //     .map(|entry| entry.expect("Error al leer entrada"))
    //     .fold(HashMap::new(), |mut acc, entry| {
    //         let name_site = entry.file_name().into_string().expect("en string");
    //         let site = Site::new();
    //         acc.insert(name_site.clone(), site);

    //         BufReader::new(File::open(entry.path()).expect("file no se puede abrir"))
    //             .lines()
    //             .for_each(|line| {
    //                 let question: Question =
    //                     serde_json::from_str(&line.expect("error al leer linea"))
    //                         .expect("Error al deserializar");
    //                 if let Some(site) = acc.get_mut(&name_site) {
    //                     site.sum_questions(1);
    //                     let words_number = question
    //                         .texts
    //                         .into_iter()
    //                         .fold(0, |acc, text| text.split_whitespace().count() + acc);
    //                     site.sum_words(words_number);
    //                 }
    //             });
    //         acc
    // });

    // forma normal
    let mut sites: HashMap<String, Site> = HashMap::new();
    let iter_directory = fs::read_dir("data1")?;

    for entry in iter_directory {
        let entry = entry?;
        let name_site = entry.file_name().to_string_lossy().to_string();
        let site = Site::new();
        sites.insert(name_site.clone(), site);
        let file = File::open(entry.path())?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let question: Question = serde_json::from_str(&line)?;
            if let Some(site) = sites.get_mut(&name_site) {
                site.sum_questions(1);
                let words_number = question
                    .texts
                    .into_iter()
                    .fold(0, |acc, text| text.split_whitespace().count() + acc);
                site.sum_words(words_number);
            }
        }
    }

    // Crear la estructura JSON
    let json_data = json!({
        "padron": 108672,
        "sites": sites,
    });

    // Convertir el objeto JSON a una cadena con formato JSON ordenado
    let formatted_json = serde_json::to_string_pretty(&json_data)?;

    // Imprimir la cadena formateada
    println!("{}", formatted_json);
    Ok(())
}
