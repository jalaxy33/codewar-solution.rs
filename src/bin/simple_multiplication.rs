/*!
 * Kata: Simple Multiplication
 * Difficulty: 8 kyu
 * Link: https://www.codewars.com/kata/583710ccaa6717322c000105/train/rust
 *
 * This kata is about multiplying a given number by eight if it is an even number
 * and by nine otherwise.
*/

#[allow(dead_code)]
fn simple_multiplication(number: u8) -> u8 {
    match number {
        n if n % 2 == 0 => n * 8,
        _ => number * 9,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(simple_multiplication(1), 9);
        assert_eq!(simple_multiplication(2), 16);
        assert_eq!(simple_multiplication(4), 32);
        assert_eq!(simple_multiplication(5), 45);
    }
}
