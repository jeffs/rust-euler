//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindrome(n: u64) -> bool {
    let digits: Vec<_> = format!("{n}").chars().collect();
    (0..=(digits.len() / 2)).all(|i| digits[i] == digits[digits.len() - i - 1])
}

fn euler4() -> u64 {
    (100..=999)
        .flat_map(move |i| (100..=999).map(move |j| i * j))
        .filter_map(|n| is_palindrome(n).then_some(n))
        .max()
        .expect("range should include at least one palindrome")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        for i in 0..=9 {
            assert!(is_palindrome(i));
        }
        assert!(!is_palindrome(10));
        assert!(is_palindrome(11));
        assert!(!is_palindrome(12));
        assert!(is_palindrome(987789));
        assert!(!is_palindrome(987689));
    }

    #[test]
    fn test_euler4() {
        assert_eq!(euler4(), 906609);
    }
}

fn main() {
    println!("{}", euler4());
}
