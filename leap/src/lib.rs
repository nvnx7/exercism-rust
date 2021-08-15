pub fn is_leap_year(year: u64) -> bool {
    // unimplemented!("true if {} is a leap year", year)
    let mut is_leap: bool = false;
    if year % 4 == 0 {
        is_leap = true;
    }

    if year % 100 == 0 && year % 400 != 0 {
        is_leap = false;
    }

    return is_leap;
}
