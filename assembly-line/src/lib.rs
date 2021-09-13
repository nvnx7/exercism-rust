// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut r = match speed {
        9..=10 => 0.77f64,
        5..=8 => 0.9f64,
        _ => 1f64,
    };
    r * (speed as f64) * 221f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
