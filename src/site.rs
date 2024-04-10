use crate::tag::Tag;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

/// It represent a site of stackexchange (a line of the data)
/// The attributes are:
///     questions: total number of questions to this site,
///     words: total number of words to this site,
///     tags: all tags for this site,
///     chatty_tags: los 10 tags con mayor relación words/questions para ese sitio,
#[derive(Deserialize, Serialize, Debug)]
pub struct Site {
    pub questions: usize,
    pub words: usize,
    pub tags: HashMap<String, Tag>,
    pub chatty_tags: Vec<String>,
}

impl Site {
    pub fn new() -> Site {
        Site {
            questions: 0,
            words: 0,
            tags: HashMap::new(),
            chatty_tags: vec![],
        }
    }
    /// add  to the number of questions
    pub fn sum_questions(&mut self, questions_number: usize) {
        self.questions += questions_number
    }

    /// add to the number of words
    pub fn sum_words(&mut self, words_number: usize) {
        self.words += words_number
    }

    /// modify values of tags
    pub fn add_tags(&mut self, tags: &HashMap<String, Tag>) {
        for e in tags.iter() {
            self.tags.insert(e.0.to_owned(), e.1.to_owned());
        }
    }

    pub fn obtain_tags(&self) -> HashMap<String, Tag> {
        self.tags.clone()
    }


    /// caculate_chatty_tags and set
    pub fn chatty_tags(&mut self) {

        let mut tag_ratios: Vec<(&String, f64)> = self
            .tags
            .iter()
            .map(|(name, tag)| (name, tag.words as f64 / tag.questions as f64))
            .collect();

        // Ordenamos el vector de tuplas por la relación number_of_words/number_of_questions en orden descendente
        tag_ratios.sort_by(|(_, ratio1), (_, ratio2)| match ratio2.partial_cmp(ratio1) {
            Some(o) => o,
            None => Ordering::Equal,
        });

        // Tomamos los primeros 10 elementos del vector
        let top_10_tags: Vec<&String> = tag_ratios.iter().take(10).map(|(name, _)| *name).collect();
       

        // seteamos los chatty tags
        self.chatty_tags = top_10_tags.iter().map(|s| s.to_string()).collect();
    }
}

impl Default for Site {
    fn default() -> Self {
        Self::new()
    }
}
