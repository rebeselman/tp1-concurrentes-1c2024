//! It represents a Tag of a question in a site of stackexchange.
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]

/// The attributes are:
///     questions:  <cantidad total de preguntas para ese tag para todos los sitios>
///     words:  <cantidad total palabras para ese tag para todos los sitios>
pub struct Tag {
    pub questions: usize,
    pub words: usize,
}

impl Tag {
    /// To obtain a new tag
    pub fn new_with(question: usize, wordss: usize) -> Self {
        Tag {
            questions: question,
            words: wordss,
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

impl Default for Tag {
    fn default() -> Self {
        Self::new_with(0, 0)
    }
}
