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

/// Returns a vector mapping each of its indexes to true if they are prime, and
/// false otherwise.  Note that indexes begin at zero!  This means that if you
/// want to know whether N is prime, you should call prime_sieve(N + 1).
pub fn prime_sieve(n: usize) -> Vec<bool> {
    let mut may_be_prime = vec![true; n];
    may_be_prime[0] = false;
    may_be_prime[1] = false;
    let mut p = 2;
    while p * p <= n {
        for i in ((p * p)..n).step_by(p) {
            may_be_prime[i] = false;
        }
        p += 1;
        if let Some(pos) = may_be_prime[p..].iter().position(|&b| b) {
            p += pos;
        } else {
            break;
        }
    }
    may_be_prime
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primes() {
        let primes: Vec<_> = Primes::new().take_while(|&p| p < 20).collect();
        assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_prime_sieve() {
        let n = 1000;
        let mut is_prime = vec![false; n];
        for p in Primes::new().take_while(|&p| (p as usize) < n) {
            is_prime[p as usize] = true;
        }
        assert_eq!(prime_sieve(n), is_prime);
    }
}
