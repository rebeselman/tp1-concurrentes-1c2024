use serde::{Deserialize, Serialize};

/// It represent a line of a file, a question of that site.
/// text -> [question, body] tag -> [tags...]
#[derive(Deserialize, Serialize, Debug)]
pub struct Question {
    pub texts: [String; 2],
    pub tags: Vec<String>,
}
