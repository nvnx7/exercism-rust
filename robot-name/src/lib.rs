extern crate rand;

use rand::prelude::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
    static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new())
);

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut rob = Self {
            name: String::new(),
        };
        rob.reset_name();
        rob
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        USED_NAMES.with(|cell| {
            let mut names = cell.borrow_mut();
            names.remove(&self.name);

            loop {
                let new_name = Self::generate_random_name();
                if !names.contains(&new_name) {
                    self.name = new_name.clone();
                    names.insert(new_name);
                    break;
                }
            }
        })
    }

    fn generate_random_name() -> String {
        let mut rng = thread_rng();
        format!(
            "{}{}{}",
            rng.gen_range('A'..='Z'),
            rng.gen_range('A'..='Z'),
            rng.gen_range(100..=999)
        )
    }
}
