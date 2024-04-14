use std::collections::BinaryHeap;

/// To obtain the top ten based on iterator
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
