//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for
//! which,
//!
//!     a² + b² = c².
//!
//! For example, 3² + 4² = 9 + 16 = 25 = 5².
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.

fn euler9() -> u64 {
    for i in 0..1000 {
        for j in (i + 1)..1000 {
            for k in (j + 1)..1000 {
                if i * i + j * j == k * k && i + j + k == 1000 {
                    return i * j * k;
                }
            }
        }
    }
    panic!("failed to find solution");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euler9() {
        assert_eq!(euler9(), 31875000);
    }
}

fn main() {
    println!("{}", euler9());
}
