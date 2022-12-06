use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    let fully_contains_count = input
        .lines()
        .map(|l| l.split(','))
        .map(|s| s.map(|i| i.split('-').map(|j| j.parse::<u16>().unwrap()).collect_tuple::<(_, _)>().unwrap()).collect_vec())
        .filter(|r| fully_contains(r[0], r[1]))
        .count();

    Solution::USize(fully_contains_count)
}

pub fn part_two(input: &str) -> Solution {
    let overlaps_count = input
        .lines()
        .map(|l| l.split(','))
        .map(|s| s.map(|i| i.split('-').map(|j| j.parse::<u16>().unwrap()).collect_tuple::<(_, _)>().unwrap()).collect_vec())
        .filter(|r| overlaps(r[0], r[1]))
        .count();

    Solution::USize(overlaps_count)
}

fn fully_contains(a1: (u16, u16), a2: (u16, u16)) -> bool {
    a2.0 >= a1.0 && a2.1 <= a1.1 || a1.0 >= a2.0 && a1.1 <= a2.1
}

fn overlaps(a1: (u16, u16), a2: (u16, u16)) -> bool {
    a1.0 <= a2.1 && a2.0 <= a1.1
}
