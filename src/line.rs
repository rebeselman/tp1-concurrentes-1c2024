use serde::{Deserialize, Serialize};

/// It represent a line of a file
#[derive(Deserialize, Serialize, Debug)]
pub struct Line {
    pub texts: Vec<String>,
    pub tags: Vec<String>,
}
