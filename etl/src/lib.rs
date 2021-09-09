use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut scores = BTreeMap::new();

    for (score, chars) in h.iter() {
        for c in chars {
            scores.insert(c.to_lowercase().next().unwrap(), *score);
        }
    }
    scores
}
