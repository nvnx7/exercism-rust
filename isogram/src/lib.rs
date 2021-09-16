use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::new();

    candidate
        .chars()
        .try_for_each(|c| match letters.insert(c.to_ascii_lowercase()) {
            true => Some(()),
            false => {
                if c.is_alphabetic() {
                    None
                } else {
                    Some(())
                }
            }
        })
        .is_some()
}
