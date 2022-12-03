use crate::solution::Solution;
use itertools::Itertools;

const LOWER_ALPH: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_ALPH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &str) -> Solution {
    let priority_sum = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|comparts| comparts.0.chars().find(|c| comparts.1.chars().contains(c)).unwrap())
        .map(|item| get_item_priority(item))
        .sum::<usize>();

    Solution::USize(priority_sum)
}

pub fn part_two(input: &str) -> Solution {
    let priority_sum = input
        .lines().chunks(3).into_iter()
        .map(|chunks| chunks.collect_tuple::<(_, _, _)>().unwrap())
        .map(|comparts| comparts.0.chars().find(|c| comparts.1.chars().contains(c) && comparts.2.chars().contains(c)).unwrap())
        .map(|badge| get_item_priority(badge))
        .sum::<usize>();

    Solution::USize(priority_sum)
}

fn get_item_priority(item: char) -> usize {
    LOWER_ALPH.chars().position(|a| a == item).unwrap_or_else(|| UPPER_ALPH.chars().position(|a| a == item).unwrap() + 26) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2022, 3);
        assert_eq!(part_one(&input), Solution::USize(7581));
        assert_eq!(part_two(&input), Solution::USize(2525));
    }
}
