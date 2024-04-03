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
use crate::tag::Tag;
pub mod config;
pub mod question;
pub mod site;
pub mod tag;
pub mod totals;

/// Function which runs the application
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    //Command::new("/bin/sh").arg("download_data.sh").output()?;
    let mut sites: HashMap<String, Site> = HashMap::new();
    let mut tags: HashMap<String, Tag> =  HashMap::new();

    let iter_directory = fs::read_dir("data1")?;

    for entry in iter_directory {
        let entry = entry?;
        let name_site = entry.file_name().to_string_lossy().to_string();

        // new site for entry
        let site = Site::new();

        // new tags for entry
        let tags_for_site: &mut HashMap<String, Tag> =  &mut HashMap::new();

        // insert new site == line in sites hashmap
        sites.insert(name_site.clone(), site);
        
        let reader = BufReader::new(File::open(entry.path())?);

        for line in reader.lines() {
            let line = line?;
            // read line in json format
            let question: Question = serde_json::from_str(&line)?;

            if let Some(site) = sites.get_mut(&name_site) {

                // add number of questions
                site.sum_questions(1);

                // calculate number of words
                let words_number = question
                    .texts
                    .into_iter()
                    .fold(0, |acc, text| text.split_whitespace().count() + acc);

                // add number of words
                site.sum_words(words_number);
                
                // add tags of this site
                question.tags.into_iter().for_each(|tag_name| {
                    if let Some(tag) = tags_for_site.get_mut(&tag_name){
                        tag.sum_questions(1);
                        tag.sum_words(words_number);

                        
                    }
                    else {
                        tags_for_site.insert(tag_name.to_owned(), Tag::new_with(1, words_number));
                    }

                    // global tag!!!
                    if let Some(tag) = tags.get_mut(&tag_name){
                        tag.sum_questions(1);
                        tag.sum_words(words_number);

                        
                    }
                    else {
                        tags.insert(tag_name, Tag::new_with(1, words_number));
                    }
                });

            }

        if let Some(site) = sites.get_mut(&name_site) {
            site.add_tags(tags_for_site);
            site.chatty_tags();
        }
    
        }
    }


    // Crear la estructura JSON
    let json_data = json!({
        "padron": 108672,
        "sites": sites,
        "tags": tags,
    });

    // Convertir el objeto JSON a una cadena con formato JSON ordenado
    let formatted_json = serde_json::to_string_pretty(&json_data)?;

    // Imprimir la cadena formateada
    println!("{}", formatted_json);
    Ok(())
}
