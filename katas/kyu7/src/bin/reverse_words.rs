/*!
 * Kata: Reverse words
 * Difficulty: 7 kyu
 * Link: https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust
 *
 * Complete the function that accepts a string parameter, and reverses each word in the string.
 * All spaces in the string should be retained.
 *
 * Examples:
 *   "This is an example!" ==> "sihT si na !elpmaxe"
 *   "double  spaces"      ==> "elbuod  secaps"
 */
#[allow(dead_code)]
fn reverse_words(str: &str) -> String {
    str.split(' ')
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {}

// Rust tests
#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
    assert_eq!(
        reverse_words("double  spaced  words"),
        "elbuod  decaps  sdrow"
    );
}
