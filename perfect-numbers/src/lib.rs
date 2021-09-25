use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => match (1..=(num / 2))
            .fold(0_u64, |sum, n| if num % n == 0 { sum + n } else { sum })
            .cmp(&num)
        {
            Ordering::Greater => Some(Classification::Abundant),
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
        },
    }
}
