/*!
 * Kata: Return Negative
 * Difficulty: 8 kyu
 * Link: https://www.codewars.com/kata/55685cd7ad70877c23000102/train/rust
 *
 * In this simple assignment you are given a number and have to make it negative.
 * But maybe the number is already negative?
 *
 * Examples:
 *   make_negative(1);  // return -1
 *   make_negative(-5); // return -5
 *   make_negative(0);  // return 0
 */

#[allow(dead_code)]
fn make_negative(n: i32) -> i32 {
    match n >= 0 {
        true => -n,
        false => n,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::make_negative;

    #[test]
    fn sample_tests() {
        assert_eq!(make_negative(1), -1);
        assert_eq!(make_negative(-5), -5);
        assert_eq!(make_negative(0), 0);
    }
}
