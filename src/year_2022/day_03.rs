use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    Solution::Str(String::from("Not implemented"))
}

pub fn part_two(input: &str) -> Solution {
    Solution::Str(String::from("Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2022, 2);
        assert_eq!(part_one(&input), Solution::I32(70613));
        assert_eq!(part_two(&input), Solution::I32(205805));
    }
}
