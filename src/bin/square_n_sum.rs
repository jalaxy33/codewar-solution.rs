/*!
 * Kata: Square(n) Sum
 * Difficulty: 8 kyu
 * Link: https://www.codewars.com/kata/56b5b908de5b8d5b8d0000f1/train/rust
 *
 * Complete the square sum function so that it squares each number passed into it
 * and then sums the results together.
 *
 * For example, for [1, 2, 2] it should return 9
 * because 1^2 + 2^2 + 2^2 = 9
 */

#[allow(dead_code)]
fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|&x| x * x).sum()
}

fn main() {}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
