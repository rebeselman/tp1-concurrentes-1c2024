use crate::tag::Tag;
use serde::{Deserialize, Serialize};

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
    pub tags: Vec<Tag>,
    //pub chatty_tags: [String; 10],
}

impl Site {
    pub fn new() -> Site {
        Site {
            questions: 0,
            words: 0,
            tags: vec![],
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
}

impl Default for Site {
    fn default() -> Self {
        Self::new()
    }
}
