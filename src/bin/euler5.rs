//! 2520 is the smallest number that can be divided by each of the numbers from
//! 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the
//! numbers from 1 to 20?

use std::collections::HashMap;

/// Stores the prime factors of a number, paired with their exponents.
struct PowerMap {
    powers: HashMap<u64, u32>,
}

impl PowerMap {
    fn new() -> PowerMap {
        PowerMap {
            powers: HashMap::new(),
        }
    }

    /// Updates this PowerMap such that it includes the prime factors of n,
    /// raised to at least the same powers as in n (in addition to whatever
    /// factors and powers this PowerMap already contained).
    fn merge_value(&mut self, mut n: u64) {
        let mut factor = 2;
        while factor <= n {
            let mut power = 0;
            while n % factor == 0 {
                power += 1;
                n /= factor;
            }
            if power > 0 {
                self.powers
                    .entry(factor)
                    .and_modify(|old_power| *old_power = (*old_power).max(power))
                    .or_insert(power);
            }
            factor += 1;
        }
    }

    fn to_value(&self) -> u64 {
        let mut value = 1;
        for (&factor, &power) in &self.powers {
            value *= factor.pow(power);
        }
        value
    }
}

/// Returns the Least Common Multiple (LCM) of the specified numbers.
fn lcm(numbers: impl IntoIterator<Item = u64>) -> u64 {
    let mut powers: PowerMap = PowerMap::new();
    for n in numbers {
        powers.merge_value(n);
    }
    powers.to_value()
}

fn euler5() -> u64 {
    lcm(2..=20)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm([]), 1);
        assert_eq!(lcm([1]), 1);
        assert_eq!(lcm([2]), 2);
        assert_eq!(lcm([1, 2]), 2);
        assert_eq!(lcm([2, 3]), 6);
        assert_eq!(lcm([18, 6, 15]), 90);
    }

    #[test]
    fn test_euler5() {
        assert_eq!(euler5(), 232792560);
    }
}

fn main() {
    println!("{}", euler5());
}
