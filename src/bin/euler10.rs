//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.

use rust_euler::prime_sieve;

fn sum_primes_below(end: u64) -> u64 {
    prime_sieve(end as usize)
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| b.then_some(i as u64))
        .sum()
}

fn euler10() -> u64 {
    sum_primes_below(2_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_primes_below() {
        assert_eq!(sum_primes_below(10), 17);
    }

    #[test]
    fn test_euler10() {
        assert_eq!(euler10(), 142913828922);
    }
}

fn main() {
    println!("{}", euler10());
}
