//! It represents the most "chatty" sites and tags of a set of files with information from StackExchange.
use crate::{site::Site, tag::Tag, utilities::top_10};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The attributes are:
///     questions: the 10 sites with most ratio words/questions
///     words: the 10 tags with most ration words/questions
#[derive(Deserialize, Serialize, Debug)]
pub struct Totals {
    pub chatty_sites: Vec<String>,
    pub chatty_tags: Vec<String>,
}

impl Totals {
    /// Creates a total based on tags and sites hashmaps.
    pub fn new_from(tags: &HashMap<String, Tag>, sites: &HashMap<String, Site>) -> Self {
        let tags_list: Vec<(String, f64)> = tags
            .iter()
            .map(|(name, tag)| (String::from(name), tag.words as f64 / tag.questions as f64))
            .collect();

        let chatty_tags_vec = top_10(tags_list);

        let sites_list: Vec<(String, f64)> = sites
            .iter()
            .map(|(name, site)| {
                (
                    String::from(name),
                    site.words as f64 / site.questions as f64,
                )
            })
            .collect();

        let chatty_sites_vec = top_10(sites_list);
        Totals {
            chatty_sites: chatty_sites_vec,
            chatty_tags: chatty_tags_vec,
        }
    }
}
