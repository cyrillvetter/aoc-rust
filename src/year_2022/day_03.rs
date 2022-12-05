use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    let priority_sum = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|comparts| comparts.0.bytes().find(|c| comparts.1.bytes().contains(c)).unwrap())
        .map(get_item_priority)
        .sum::<u32>();

    Solution::U32(priority_sum)
}

pub fn part_two(input: &str) -> Solution {
    let priority_sum = input
        .lines().chunks(3).into_iter()
        .map(|chunks| chunks.collect_tuple::<(_, _, _)>().unwrap())
        .map(|comparts| comparts.0.bytes().find(|c| comparts.1.bytes().contains(c) && comparts.2.bytes().contains(c)).unwrap())
        .map(get_item_priority)
        .sum::<u32>();

    Solution::U32(priority_sum)
}

fn get_item_priority(item: u8) -> u32 {
    if item > 96 { item as u32 - 96 } else { item as u32 - 38 }
}
