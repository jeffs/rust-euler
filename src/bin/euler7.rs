//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

use rust_euler::nth_prime;

fn euler7() -> u64 {
    nth_prime(10_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(nth_prime(5), 13);
    }

    #[test]
    fn test_euler7() {
        assert_eq!(euler7(), 104743);
    }
}

fn main() {
    println!("{}", euler7());
}
