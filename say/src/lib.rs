pub fn encode(n: u64) -> String {
    match n {
        x @ 0..=999 => convert_smaller_number(x),
        _ => {
            let mut rev_digits = n.to_string().chars().rev().collect::<Vec<_>>();
            rev_digits
                .chunks_mut(3)
                .rev()
                .map(|chunk| chunk.iter().rev().collect::<String>())
                .rev()
                .enumerate()
                .fold(String::new(), |acc, (i, x)| {
                    let prefix = if let Ok(n) = x.trim_start_matches('0').parse::<u64>() {
                        convert_smaller_number(n)
                    } else {
                        "".to_string()
                    };
                    let suffix = if prefix.is_empty() {
                        "".to_string()
                    } else {
                        match i {
                            0 => String::new(),
                            1 => String::from(" thousand "),
                            2 => String::from(" million "),
                            3 => String::from(" billion "),
                            4 => String::from(" trillion "),
                            5 => String::from(" quadrillion "),
                            6 => String::from(" quintillion "),
                            _ => panic!("Number out of range!"),
                        }
                    };
                    format!("{}{}{}", prefix, suffix, acc)
                })
                .trim()
                .to_string()
        }
    }
}

fn convert_smaller_number(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        x @ 14..=19 => match x {
            15 => "fifteen".to_string(),
            18 => "eighteen".to_string(),
            _ => format!("{}teen", convert_smaller_number(x - 10)),
        },
        20 => String::from("twenty"),
        x @ 21..=29 => format!("twenty-{}", convert_smaller_number(x - 20)),
        30 => String::from("thirty"),
        x @ 31..=39 => format!("thirty-{}", convert_smaller_number(x - 30)),
        40 => String::from("forty"),
        x @ 41..=49 => format!("forty-{}", convert_smaller_number(x - 40)),
        50 => String::from("fifty"),
        x @ 51..=59 => format!("fifty-{}", convert_smaller_number(x - 50)),
        60 => String::from("sixty"),
        x @ 61..=69 => format!("sixty-{}", convert_smaller_number(x - 60)),
        70 => String::from("seventy"),
        x @ 71..=79 => format!("seventy-{}", convert_smaller_number(x - 70)),
        80 => String::from("eighty"),
        x @ 81..=89 => format!("eighty-{}", convert_smaller_number(x - 80)),
        90 => String::from("ninety"),
        x @ 91..=99 => format!("ninety-{}", convert_smaller_number(x - 90)),
        100 => String::from("one hundred"),
        x @ 101..=999 => format!(
            "{} hundred {}",
            convert_smaller_number(x / 100),
            convert_smaller_number(x % 100)
        ),
        _ => panic!("Out of range!"),
    }
}
