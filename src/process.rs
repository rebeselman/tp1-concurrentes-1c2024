use crate::question::Question;
use crate::site::Site;
use crate::tag::Tag;
use crate::utilities::merge_tag_maps;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::io::{self, BufReader, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;
use std::{fs::File, io::BufRead};

/// Fuction to process files in parallel
pub fn process_files_in_parallel(
    filenames: Vec<PathBuf>,
    number_of_threads: usize,
) -> io::Result<(HashMap<String, Site>, HashMap<String, Tag>)> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(number_of_threads)
        .build_global()
        .map_err(|_| ErrorKind::Other)?;

    let mut sites = HashMap::new();
    let mut tags: HashMap<String, Tag> = HashMap::new();

    for path in filenames {
        let start = Instant::now();
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let (word_number, question_number, tags_for_site) = process_file(reader)?;
        let name_site = path
            .file_name()
            .map_or_else(String::new, |s| s.to_string_lossy().to_string());
        if !name_site.is_empty() {
            // add tags

            tags_for_site.iter().for_each(|(tag_name, tag)| {
                tags.entry(tag_name.to_string())
                    .and_modify(|t| {
                        t.sum_questions(tag.questions);
                        t.sum_words(tag.words)
                    })
                    .or_insert(tag.to_owned());
            });

            // add site
            let mut site = Site::new_with(question_number, word_number, tags_for_site);
            site.chatty_tags();
            sites.insert(name_site, site);

            println!("sites se procesó en:{:?}", start.elapsed());
        }
    }
    Ok((sites, tags))
}

/// Returns a  (words_number, questions_number, hash map of tags) from a file
fn process_file(
    reader: BufReader<File>,
) -> Result<(usize, usize, HashMap<String, Tag>), io::Error> {
    // tal vez conviene pasarle el bufer reader y que lea de a poco y abre menos archivos paralelamente
    let results = reader
        .lines()
        .par_bridge()
        .map(|l| match l {
            Ok(line) => process_line(line),
            Err(_) => (0, 0, HashMap::new()),
        })
        .filter(|res| !res.2.is_empty())
        .reduce(
            || (0, 0, HashMap::new()),
            |acc, res| {
                let merged_tags: HashMap<String, Tag> = merge_tag_maps(acc.2, res.2);
                (res.0 + acc.0, res.1 + acc.1, merged_tags)
            },
        );

    Ok(results)
}

/// Process a line and returns -> (words_number, questions_number, hash map of tags)
fn process_line(line: String) -> (usize, usize, HashMap<String, Tag>) {
    //let start = Instant::now();
    match serde_json::from_str::<Question>(&line) {
        Ok(question) => {
            // cuento cantidad de palabras para esta pregunta
            let words_number = question
                .texts
                .par_iter()
                .map(|c| c.split_whitespace().count())
                .sum();
            // obtengo cantidad de preguntas hasta ahora

            let mut hash_tag = HashMap::with_capacity(question.tags.len());
            for tag in &question.tags {
                hash_tag.insert(tag.clone(), Tag::new_with(1, words_number));
            }

            //println!("tiempo para procesar linea: {:?}", start.elapsed());
            (words_number, 1, hash_tag)
        }
        // si hay error simplemente no se cuenta nada y luego se filtra
        Err(_) => (0, 0, HashMap::new()),
    }
}
