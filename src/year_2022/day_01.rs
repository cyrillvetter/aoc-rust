use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    let result = input
        .split("\r\n\r\n")
        .map(|v| v.lines().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .max().unwrap();

    Solution::I32(result)
}

pub fn part_two(input: &str) -> Solution {
    let result = input
        .split("\r\n\r\n")
        .map(|v| v.lines().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .sorted_by(|a, b| Ord::cmp(&b, &a)) // Sort in descending order
        .take(3)
        .sum::<i32>();

    Solution::I32(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2022, 1);
        assert_eq!(part_one(&input), Solution::I32(70613));
        assert_eq!(part_two(&input), Solution::I32(205805));
    }
}
