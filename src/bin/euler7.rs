//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6th prime is 13.
//!
//! What is the 10 001st prime number?

/// An iterator over the prime numbers.
struct Primes {
    known: Vec<u64>,
}

impl Primes {
    fn new() -> Primes {
        Primes { known: Vec::new() }
    }

    fn is_prime(&self, n: u64) -> bool {
        !self.known.iter().any(|p| n % p == 0)
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.known.last().cloned() {
            None => {
                self.known.push(2);
            }
            Some(2) => {
                self.known.push(3);
            }
            Some(mut n) => {
                while {
                    n += 2;
                    !self.is_prime(n)
                } {}
                self.known.push(n);
            }
        }
        self.known.last().cloned()
    }
}

fn nth_prime(n: usize) -> u64 {
    Primes::new()
        .nth(n)
        .expect("primes should be inexhaustible")
}

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
