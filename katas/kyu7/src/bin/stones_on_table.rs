/*!
 * Kata: Stones on the Table
 * Difficulty: 7 kyu
 * Link: https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a
 *
 * There are some stones on Bob's table in a row, and each of them can be red, green or blue,
 * indicated by the characters R, G, and B.
 * Help Bob find the minimum number of stones he needs to remove from the table
 * so that the stones in each pair of adjacent stones have different colors.
 *
 * Examples:
 *   "RGBRGBRGGB"   => 1
 *   "RGGRGBBRGRR"  => 3
 *   "RRRRGGGGBBBB" => 9
 */
#[allow(dead_code)]
fn stones_to_remove(stones: &str) -> usize {
    stones
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|pair| pair[0] == pair[1])
        .count()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::stones_to_remove;

    #[test]
    fn sample_tests() {
        assert_eq!(stones_to_remove("RGBRGBRGGB"), 1);
        assert_eq!(stones_to_remove("RGGRGBBRGRR"), 3);
        assert_eq!(stones_to_remove("RRRRGGGGBBBB"), 9);
    }
}
