//! Hi! :), this is my implementation of the tp1
use config::Config;

use question::Question;
use serde_json::{self, json};
use site::Site;
use tag::Tag;

use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::thread;

pub mod config;
pub mod question;
pub mod site;
pub mod tag;
pub mod totals;

// Función para procesar archivos en paralelo y recolectar los resultados
fn process_files_in_parallel(
    filenames: Vec<PathBuf>,
    number_of_threads: usize,
) -> io::Result<HashMap<String, Site>> {
    //let worklists = split_vec_into_chunks(filenames, number_of_threads);

    let worklists = split_vec_into_chunks(filenames, number_of_threads);

    let mut thread_handles = vec![];

    for worklist in worklists {
        thread_handles.push(thread::spawn(move || process_files(worklist)));
    }

    let mut sites = HashMap::new();

    for handle in thread_handles {
        if let Ok(r) = handle.join() {
            sites.extend(r);
        } else {
            // Manejar errores de los threads si es necesario
        }
    }

    Ok(sites)
}

// Función para dividir un vector en chunks para procesamiento en paralelo
fn split_vec_into_chunks<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for item in vec {
        chunk.push(item.clone());
        if chunk.len() == chunk_size {
            result.push(chunk.clone());
            chunk.clear();
        }
    }

    if !chunk.is_empty() {
        result.push(chunk);
    }

    result
}

// Función ficticia para simular el procesamiento de archivos y la creación de objetos Site
fn process_files(worklist: Vec<PathBuf>) -> HashMap<String, Site> {
    // Aquí deberías realizar el procesamiento real de los archivos y crear objetos Site
    // En esta versión de ejemplo, simplemente se devuelve un objeto Site ficticio
    let sites: HashMap<String, Site> = worklist
        .iter()
        .map(|path| {
            let site = match process_file(path.to_path_buf()) {
                Ok(site) => site,
                Err(_) => Site::new(),
            };
            let name_site = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => String::new(),
            };

            (name_site, site)
        })
        .filter(|tuple| !tuple.0.is_empty())
        .collect();
    sites
}

/// Function which runs the application
pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    //Command::new("/bin/sh").arg("download_data.sh").output()?;

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

    println!("{:?}", file_paths);
    // Procesar los archivos en paralelo y obtener los sitios
    let sites = process_files_in_parallel(file_paths, c.number_of_threads)?;

    // Crear la estructura JSON
    let json_data = json!({
        "padron": 108672,
        "sites": sites,
    });
    //let mut file = File::create("cosa-horrorosa")?;

    // Convertir el objeto JSON a una cadena con formato JSON ordenado
    let formatted_json = serde_json::to_string_pretty(&json_data)?;
    //file.write_all(formatted_json.as_bytes())?;
    println!("{}", formatted_json);
    Ok(())
}

/// Returns a Site from a file
fn process_file(path: PathBuf) -> Result<Site, io::Error> {
    let file = File::open(path)?;
    // abro buffer para leer linea a linea
    let reader = BufReader::new(file);
    let results = reader
        .lines()
        .map(|l| match l {
            Ok(line) => process_line(line),
            Err(_) => (0, 0, HashMap::new()),
        })
        .filter(|res| !res.2.is_empty())
        .fold((0, 0, HashMap::new()), |mut acc, res| {
            acc.2.extend(res.2);
            (res.0 + acc.0, res.1 + acc.1, acc.2)
        });

    Ok(Site {
        questions: results.1,
        words: results.0,
        tags: results.2,
        chatty_tags: vec![],
    })
}

/// Procesa la línea y devuelve para ella: (cantidad de palabras, cantidad de preguntas, hash con todos los tags para dicho sitio)
fn process_line(line: String) -> (usize, usize, HashMap<String, Tag>) {
    match serde_json::from_str::<Question>(&line) {
        Ok(question) => {
            // cuento cantidad de palabras para esta pregunta
            let words_number = question
                .texts
                .into_iter()
                .fold(0, |acc, text| text.split_whitespace().count() + acc);
            // obtengo cantidad de preguntas hasta ahora
            let question_number: usize = 1;
            let hash_tags = question
                .tags
                .into_iter()
                .fold(HashMap::new(), |mut acc, tag| {
                    acc.entry(tag)
                        .and_modify(|tag: &mut Tag| {
                            tag.sum_questions(1);
                            tag.sum_words(words_number)
                        })
                        .or_insert_with_key(|_| Tag::new_with(1, words_number));
                    acc
                });
            (words_number, question_number, hash_tags)
        }
        // si hay error simplemente no se cuenta nada y luego se filtra
        Err(_) => (0, 0, HashMap::new()),
    }
}
