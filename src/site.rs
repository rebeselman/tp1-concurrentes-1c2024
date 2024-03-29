use serde::{Deserialize, Serialize};
use crate::tag::Tag;

/// It represent a site of stackexchange (a line of the data)
/// The attributes are:
///     questions: total number of questions to this site,
///     words: total number of words to this site,
///     tags: all tags for this site,
///     chatty_tags: los 10 tags con mayor relación words/questions para ese sitio,
#[derive(Deserialize, Serialize, Debug)]
pub struct Site {
    pub questions: u32,
    pub words: u32,
    pub tags: Vec<Tag>,
    pub chatty_tags: [String; 10],
}