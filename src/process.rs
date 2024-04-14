use crate::question::Question;
use crate::site::Site;
use crate::tag::Tag;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use std::io::{self, BufReader, ErrorKind};
use std::path::PathBuf;
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

/// Process a line and returns -> (words_number, questions_number, hash map of tags)
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
