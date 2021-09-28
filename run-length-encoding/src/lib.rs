pub fn encode(source: &str) -> String {
    let mut data = String::new();
    let mut count: usize = 0;
    let mut last_char: char = source.chars().next().unwrap_or_default();

    for c in source.chars().chain(std::iter::once('\0')) {
        if c == last_char {
            count += 1;
        } else {
            match count {
                1 => data.push_str(&format!("{}", last_char)),
                _ => data.push_str(&format!("{}{}", count, last_char)),
            }
            count = 1;
            last_char = c;
        }
    }

    data
}

pub fn decode(source: &str) -> String {
    let mut data = String::new();
    let mut count_str = String::new();

    for c in source.chars() {
        if c.is_digit(10) {
            count_str.push(c);
        } else {
            let count: usize = count_str.parse().unwrap_or(1);
            data.push_str(&c.to_string().repeat(count));
            count_str = String::new();
        }
    }

    data
}
