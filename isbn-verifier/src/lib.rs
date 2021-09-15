/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    match isbn
        .chars()
        .filter(|&c| c != '-')
        .enumerate()
        .map(|(i, c)| match (i, c) {
            (idx, '0'..='9') => Some((idx, c.to_digit(10).unwrap() * (10 - (idx as u32)))),
            (9, 'X') => Some((9, 10)),
            _ => None,
        })
        .try_fold((0, 0), |(_, acc), opt| opt.map(|(idx, n)| (idx, acc + n)))
    {
        Some((9, s)) => s % 11 == 0,
        _ => false,
    }
}
