#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > u8::max(capacity_1, capacity_2) || goal % gcd(capacity_1, capacity_2) != 0 {
        None
    } else {
        let (start_cap, other_cap) = if let Bucket::One = start_bucket {
            (capacity_1, capacity_2)
        } else {
            (capacity_2, capacity_1)
        };

        let mut moves = 1_u8;

        let mut start_amt = start_cap;
        let mut other_amt = 0_u8;

        if other_cap == goal {
            other_amt = other_cap;
            moves += 1;
        }

        let mut max_pour: u8;
        while start_amt != goal && other_amt != goal {
            max_pour = u8::min(start_amt, other_cap - other_amt);
            other_amt += max_pour;
            start_amt -= max_pour;

            moves += 1;
            if start_amt == goal {
                break;
            }

            if other_amt == goal {
                break;
            }
            if start_amt == 0 {
                start_amt = start_cap;
                moves += 1;
            }
            if other_cap == other_amt {
                other_amt = 0;
                moves += 1;
            }
        }

        Some(BucketStats {
            moves,
            goal_bucket: if start_amt == goal && start_bucket == &Bucket::One {
                Bucket::One
            } else {
                Bucket::Two
            },
            other_bucket: if start_amt == goal {
                other_amt
            } else {
                start_amt
            },
        })
    }
}

fn gcd(x: u8, y: u8) -> u8 {
    let max: u8 = u8::max(x, y);
    let min: u8 = u8::min(x, y);

    if min == 0 {
        max
    } else {
        gcd(min, max % min)
    }
}
