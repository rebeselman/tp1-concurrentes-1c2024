use std::collections::HashMap;

/// It represents an abstract of the sites of stackexchange
/// The attributes are:
///     questions: los 10 sitios con mayor relación words/questions
///     words: los 10 tags con mayor relación words/questions entre todos los sitios
use serde::{Deserialize, Serialize};

use crate::{site::Site, tag::Tag, utilities::top_10};
#[derive(Deserialize, Serialize, Debug)]
pub struct Totals {
    pub chatty_sites: Vec<String>,
    pub chatty_tags: Vec<String>,
}

impl Totals {
    pub fn new_from(tags: &HashMap<String, Tag>, sites: &HashMap<String, Site>) -> Self {
        let tags_iter = tags.iter().map(|(name, site)| {
            (
                site.words as u32 / site.questions as u32,
                String::from(name),
            )
        });

        let chatty_tags_vec = top_10(tags_iter);

        let sites_iter = sites.iter().map(|(name, site)| {
            (
                site.words as u32 / site.questions as u32,
                String::from(name),
            )
        });

        let chatty_sites_vec = top_10(sites_iter);
        Totals {
            chatty_sites: chatty_sites_vec,
            chatty_tags: chatty_tags_vec,
        }
    }
}
