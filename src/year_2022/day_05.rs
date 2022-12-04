use crate::solution::Solution;

pub fn part_one(input: &str) -> Solution {
    Solution::Empty
}

pub fn part_two(input: &str) -> Solution {
    Solution::Empty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2022, 5);
        assert_eq!(part_one(&input), Solution::Empty);
        assert_eq!(part_two(&input), Solution::Empty);
    }
}
