use crate::tag::Tag;
use std::collections::{BinaryHeap, HashMap};

/// To split two vectors in chunks to process in parallel
pub fn split_vec_into_chunks<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for item in vec {
        chunk.push(item.clone());
        if chunk.len() == chunk_size {
            result.push(chunk.clone());
            chunk.clear();
        }
    }

    if !chunk.is_empty() {
        result.push(chunk);
    }

    result
}

/// Function to merge two hashmaps and its Tags
pub fn merge_tag_maps(
    map1: HashMap<String, Tag>,
    map2: HashMap<String, Tag>,
) -> HashMap<String, Tag> {
    let merged_tag = map1.iter().fold(map2, |mut acc, tag| {
        acc.entry(tag.0.to_owned())
            .and_modify(|t| {
                t.sum_questions(tag.1.questions);
                t.sum_words(tag.1.words)
            })
            .or_insert(tag.1.clone());
        acc
    });
    merged_tag
}

pub fn top_10(iter: impl Iterator<Item = (u32, String)>) -> Vec<String> {
    let mut heap = BinaryHeap::new();

    for (ratio, name) in iter {
        if heap.len() < 10 {
            heap.push((ratio, name));
        } else if let Some(t) = heap.peek() {
            if ratio > t.0 {
                heap.pop();
                heap.push((ratio, name));
            }
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .map(|x| x.1.clone())
        .collect()
}
