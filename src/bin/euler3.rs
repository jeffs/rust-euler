//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidate = 2;
    while candidate <= n {
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }
    factors
}

fn euler3() -> u64 {
    *prime_factors(600851475143)
        .last()
        .expect("value should have at least one prime factor")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(1), []);
        assert_eq!(prime_factors(2), [2]);
        assert_eq!(prime_factors(3), [3]);
        assert_eq!(prime_factors(4), [2, 2]);
        assert_eq!(prime_factors(5), [5]);
        assert_eq!(prime_factors(6), [2, 3]);
    }

    #[test]
    fn test_euler3() {
        assert_eq!(euler3(), 6857);
    }
}

fn main() {
    println!("{}", euler3());
}
