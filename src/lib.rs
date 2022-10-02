/// An iterator over the prime numbers.
pub struct Primes {
    known: Vec<u64>,
}

impl Primes {
    pub fn new() -> Primes {
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

pub fn nth_prime(n: usize) -> u64 {
    Primes::new()
        .nth(n)
        .expect("primes should be inexhaustible")
}
