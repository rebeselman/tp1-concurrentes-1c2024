use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
/// It represents a Tag of a question in a site of stackexchange.
/// The attributes are:
///     questions:  <cantidad total de preguntas para ese tag para todos los sitios>
///     words:  <cantidad total palabras para ese tag para todos los sitios>
pub struct Tag {
    pub questions: u32,
    pub words: u32
}
