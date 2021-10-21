use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    words
        .replace(|c: char| !c.is_alphanumeric() && c != '\'', " ")
        .replace("\' ", " ")
        .replace(" \'", " ")
        .to_ascii_lowercase()
        .split_whitespace()
        .for_each(|word| *counts.entry(word.to_owned()).or_insert(0) += 1);

    counts
}
