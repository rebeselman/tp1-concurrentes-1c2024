use crate::{tag::Tag, utilities::top_10};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Create new Site tags
    pub fn new() -> Site {
        Site {
            questions: 0,
            words: 0,
            tags: HashMap::new(),
            chatty_tags: vec![],
        }
    }
    pub fn new_with(
        num_questions: usize,
        num_words: usize,
        tag_hash: HashMap<String, Tag>,
    ) -> Site {
        Site {
            questions: num_questions,
            words: num_words,
            tags: tag_hash,
            chatty_tags: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.questions == 0)
            & (self.words == 0)
            & (self.tags.is_empty())
            & (self.chatty_tags.is_empty())
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
    // pub fn add_tags(&mut self, tags: &HashMap<String, Tag>) {
    //     for e in tags.iter() {
    //         self.tags.insert(e.0.to_owned(), e.1.to_owned());
    //     }
    // }

    pub fn obtain_tags(&self) -> HashMap<String, Tag> {
        self.tags.clone()
    }

    /// Caculate_chatty_tags and set
    pub fn chatty_tags(&mut self) {
        let iter = self
            .tags
            .iter()
            .map(|(name, tag)| (tag.words as u32 / tag.questions as u32, String::from(name)));
        self.chatty_tags = top_10(iter);
    }
}

impl Default for Site {
    fn default() -> Self {
        Self::new()
    }
}
