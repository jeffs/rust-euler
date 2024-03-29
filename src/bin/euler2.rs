//! Each new term in the Fibonacci sequence is generated by adding the previous
//! two terms. By starting with 1 and 2, the first 10 terms will be:
//!
//!     1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//!
//! By considering the terms in the Fibonacci sequence whose values do not
//! exceed four million, find the sum of the even-valued terms.

use std::iter::successors;

fn fibonaccis() -> impl Iterator<Item = u64> {
    successors(Some((1, 2)), |&(a, b)| Some((b, a + b))).map(|(a, _)| a)
}

fn euler2() -> u64 {
    fibonaccis()
        .filter(|n| n % 2 == 0)
        .take_while(|&n| n <= 4_000_000)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonaccis() {
        let first10: Vec<_> = fibonaccis().take(10).collect();
        assert_eq!(first10, [1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    #[test]
    fn test_euler2() {}
}

fn main() {
    println!("{}", euler2());
}
