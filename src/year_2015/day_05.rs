use crate::solution::Solution;
use std::str;

pub fn part_one(input: &str) -> Solution {
    let vowels: [char; 5] = [ 'a', 'e', 'i', 'o', 'u' ];
    let disallowed: [&str; 4] = [ "ab", "cd", "pq", "xy" ];

    let mut count: u32 = 0;
    for l in input.lines() {
        if !disallowed.iter().any(|f| l.contains(f)) &&
           l.split(vowels).count() > 3 &&
           l.as_bytes().windows(2).any(|t| t[0] == t[1]) {
            count += 1;
        }
    };

    Solution::U32(count)
}

pub fn part_two(input: &str) -> Solution {
    let mut count: u32 = 0;
    for l in input.lines() {
        if l.as_bytes().windows(3).any(|pair| pair[0] == pair[2]) &&
           // TODO: Find better way to check for repeating chars (replace rfind and as_bytes)
           l.as_bytes().windows(2).enumerate().any(|(i, pair)| l.rfind(str::from_utf8(pair).unwrap()).map(|j| j > i + 1).unwrap_or(false)) {
            count += 1;
        }
    };

    Solution::U32(count)
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use super::*;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2015, 5);
        assert_eq!(part_one(&input), Solution::U32(258));
        assert_eq!(part_two(&input), Solution::U32(53));
    }
}
