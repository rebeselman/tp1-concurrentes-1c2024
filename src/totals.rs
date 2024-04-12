use std::{cmp::Ordering, collections::HashMap};

/// It represents an abstract of the sites of stackexchange
/// The attributes are:
///     questions: los 10 sitios con mayor relación words/questions
///     words: los 10 tags con mayor relación words/questions entre todos los sitios
use serde::{Deserialize, Serialize};

use crate::{site::Site, tag::Tag};
#[derive(Deserialize, Serialize, Debug)]
pub struct Totals {
    pub chatty_sites: Vec<String>,
    pub chatty_tags: Vec<String>,
}

impl Totals {
    pub fn new_from(tags: &HashMap<String, Tag>, sites: &HashMap<String, Site>) -> Self {
        let mut tag_ratios: Vec<(&String, f64)> = tags
            .iter()
            .map(|(name, tag)| (name, tag.words as f64 / tag.questions as f64))
            .collect();

        // Ordenamos el vector de tuplas por la relación number_of_words/number_of_questions en orden descendente
        tag_ratios.sort_by(
            |(_, ratio1), (_, ratio2)| match ratio2.partial_cmp(ratio1) {
                Some(o) => o,
                None => Ordering::Equal,
            },
        );

        // Tomamos los primeros 10 elementos del vector
        let top_10_tags: Vec<&String> = tag_ratios.iter().take(10).map(|(name, _)| *name).collect();

        // seteamos los chatty tags
        let chatty_tags = top_10_tags.iter().map(|s| s.to_string()).collect();

        // hago lo mismo para sites

        let mut sites_ratios: Vec<(&String, f64)> = sites
            .iter()
            .map(|(name, site)| (name, site.words as f64 / site.questions as f64))
            .collect();

        // Ordenamos el vector de tuplas por la relación number_of_words/number_of_questions en orden descendente
        sites_ratios.sort_by(
            |(_, ratio1), (_, ratio2)| match ratio2.partial_cmp(ratio1) {
                Some(o) => o,
                None => Ordering::Equal,
            },
        );

        // Tomamos los primeros 10 elementos del vector
        let top_10_sites: Vec<&String> = sites_ratios
            .iter()
            .take(10)
            .map(|(name, _)| *name)
            .collect();

        // seteamos los chatty tags
        let chatty_sites = top_10_sites.iter().map(|s| s.to_string()).collect();

        Totals {
            chatty_sites: chatty_sites,
            chatty_tags: chatty_tags,
        }
    }
}
