pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let idx = consonant_idx(word);
            let (suffix, prefix) = word.split_at(idx);
            format!("{}{}ay", prefix, suffix)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn consonant_idx(w: &str) -> usize {
    if w.starts_with('a')
        || w.starts_with('e')
        || w.starts_with('i')
        || w.starts_with('o')
        || w.starts_with('u')
        || w.starts_with("xr")
        || w.starts_with("yt")
    {
        0
    } else if w.starts_with("qu") {
        2
    } else {
        consonant_idx(&w[1..]) + 1
    }
}
