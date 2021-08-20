pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let p = digits.len() as u32;
    digits.map(|d| u32::pow(d, p)).sum::<u32>() == num
}
