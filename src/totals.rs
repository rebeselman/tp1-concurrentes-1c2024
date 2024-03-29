
/// It represents an abstract of the sites of stackexchange
/// The attributes are:
///     questions: los 10 sitios con mayor relación words/questions
///     words: los 10 tags con mayor relación words/questions entre todos los sitios
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Totals {
    pub chatty_sites: [String; 10],
    pub chatty_tags: [String; 10]
}
