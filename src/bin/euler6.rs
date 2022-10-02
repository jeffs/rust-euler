//! The sum of the squares of the first ten natural numbers is,
//!
//!     1² + 2² + ... + 10² = 385
//!
//! The square of the sum of the first ten natural numbers is,
//!
//!     (1 + 2 + ... + 10)² = 55² = 3025
//!
//! Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is 3025 - 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.

/// Returns the sum of the squares of all natural numbers less than n.
fn sum_squares(n: u64) -> u64 {
    (1..n).map(|i| i * i).sum()
}

/// Returns the square of the sum of all natural numbers less than n.
fn square_sum(n: u64) -> u64 {
    let sum: u64 = (1..n).sum();
    sum * sum
}

fn euler6() -> u64 {
    square_sum(101) - sum_squares(101)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euler6() {
        assert_eq!(euler6(), 25164150);
    }
}

fn main() {
    println!("{}", euler6());
}
