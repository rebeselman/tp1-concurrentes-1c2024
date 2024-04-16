use std::cmp::Ordering;

/// To obtain the top ten based on a vector
pub fn top_10(mut list: Vec<(String, f64)>) -> Vec<String> {
    list.sort_by(
        |(_, ratio1), (_, ratio2)| match ratio2.partial_cmp(ratio1) {
            Some(o) => o,
            None => Ordering::Equal,
        },
    );

    list.iter()
        .take(10)
        .map(|(name, _)| String::from(name))
        .collect()
}
