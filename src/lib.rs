/*!
 * Example code from codewars doc:
 * https://docs.codewars.com/languages/rust
 */

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_i32() {
        assert_eq!(add(1, 1), 2);
    }
}