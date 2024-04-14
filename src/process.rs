use crate::question::Question;
use crate::site::Site;
use crate::tag::Tag;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::io::{self, BufReader, ErrorKind};
use std::path::PathBuf;
//use std::time::Instant;
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

    let (sites, tags) = filenames
        .par_iter()
        .map(|p| match File::open(p) {
            Ok(f) => {
                let (words_num, question_num, hash_tags) = process_file(BufReader::new(f));
                let mut site = Site::new_with(question_num, words_num, hash_tags.to_owned());

                let name_site = p
                    .file_name()
                    .map_or_else(String::new, |s| s.to_string_lossy().to_string());

                site.chatty_tags();
                let mut sites_hash = HashMap::new();
                sites_hash.insert(name_site, site);

                (sites_hash, hash_tags)
            }
            Err(_) => (HashMap::new(), HashMap::new()),
        })
        .reduce(
            || (HashMap::new(), HashMap::new()),
            |(mut sites_a, mut tags_a), (sites_b, tags_b)| {
                sites_a.extend(sites_b);
                tags_b.iter().for_each(|(tag_name, tag)| {
                    tags_a
                        .entry(tag_name.to_string())
                        .and_modify(|t: &mut Tag| {
                            t.sum_questions(tag.questions);
                            t.sum_words(tag.words);
                        })
                        .or_insert(tag.to_owned());
                });
                (sites_a, tags_b)
            },
        );

    Ok((sites, tags))
}

/// Returns a  (words_number, questions_number, hash map of tags) from a file
fn process_file(reader: BufReader<File>) -> (usize, usize, HashMap<String, Tag>) {
    //let start = Instant::now();
    // tal vez conviene pasarle el bufer reader y que lea de a poco y abre menos archivos paralelamente
    let results = reader
        .lines()
        .par_bridge()
        .map(|l| match l {
            Ok(line) => process_line(line),
            Err(_) => (0, 0, HashMap::new()),
        })
        .reduce(
            || (0, 0, HashMap::new()),
            |(w_a, q_a, mut tags_a), (w_b, q_b, tags_b)| {
                //let start = Instant::now();
                //let merged_tags: HashMap<String, Tag> = merge_tag_maps(acc.2, res.2);
                tags_b.iter().for_each(|(tag_name, tag)| {
                    tags_a
                        .entry(tag_name.to_string())
                        .and_modify(|t: &mut Tag| {
                            t.sum_questions(tag.questions);
                            t.sum_words(tag.words);
                        })
                        .or_insert(tag.to_owned());
                });

                //println!("tiempo en mergear tags:{:?}", start.elapsed());
                (w_a + w_b, q_a + q_b, tags_a)
            },
        );
    //println!("tiempo en procesar info de sites {:?}", start.elapsed());

    // match results{
    //     Some(r) => r,
    //     None => (0,0, HashMap::new())
    // }
    results
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
