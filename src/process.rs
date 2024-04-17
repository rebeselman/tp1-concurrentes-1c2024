use crate::question::Question;
use crate::site::Site;
use crate::tag::Tag;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::io::{self, BufReader, ErrorKind};
use std::path::PathBuf;
use std::{fs::File, io::BufRead};

/// Fuction to process a vector of filenames in parallel with a number of threads.
/// Returns -> (Hahsmap of sites, Hashmap of tags)
/// Where a the key is the name of that site or tag.
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

/// Process a file
/// Returns -> (number of words of the file, number of questions of the file,  HashMap of Tags of the file)
fn process_file(reader: BufReader<File>) -> (usize, usize, HashMap<String, Tag>) {
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
                tags_b.iter().for_each(|(tag_name, tag)| {
                    tags_a
                        .entry(tag_name.to_string())
                        .and_modify(|t: &mut Tag| {
                            t.sum_questions(tag.questions);
                            t.sum_words(tag.words);
                        })
                        .or_insert(tag.to_owned());
                });
                (w_a + w_b, q_a + q_b, tags_a)
            },
        );
    results
}

/// Process a line and returns -> (number of words in line, number of lines in a line, hash map of tags of that line)
fn process_line(line: String) -> (usize, usize, HashMap<String, Tag>) {
    match serde_json::from_str::<Question>(&line) {
        Ok(question) => {
            let words_number = question
                .texts
                .par_iter()
                .map(|c| c.split_whitespace().count())
                .sum();

            let mut hash_tag = HashMap::with_capacity(question.tags.len());
            for tag in &question.tags {
                hash_tag.insert(tag.clone(), Tag::new_with(1, words_number));
            }

            (words_number, 1, hash_tag)
        }

        Err(_) => (0, 0, HashMap::new()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        let json_data = String::from("{\"texts\": [\"why aliens exist?\", \"i wonder why aliens exist\"], \"tags\": [\"aliens\", \"ovni\", \"mars\"]}");
        let (num_words, num_questions, hash_tags) = process_line(json_data);
        let mut tags: HashMap<String, Tag> = HashMap::new();
        tags.insert(String::from("aliens"), Tag::new_with(1, 8));
        tags.insert(String::from("ovni"), Tag::new_with(1, 8));
        tags.insert(String::from("mars"), Tag::new_with(1, 8));
        assert_eq!(num_words, 8);
        assert_eq!(num_questions, 1);
        assert_eq!(hash_tags, tags);      
    }
    
    #[test]
    fn test_process_file() {
        // Simular diferentes situaciones de concurrencia y verificar el resultado
        // utilizando assert_eq!() o cualquier otra macro de aserción.
    }
    
    #[test]
    fn test_process_files() {
        // Ejecutar la aplicación completa y verificar el resultado final
        // utilizando assert_eq!() o cualquier otra macro de aserción.
    }
    
}