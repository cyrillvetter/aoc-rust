use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    Solution::USize(get_message_start(input, 4))
}

pub fn part_two(input: &str) -> Solution {
    Solution::USize(get_message_start(input, 14))
}

fn get_message_start(input: &str, size: usize) -> usize {
    for (i, w) in input.as_bytes().windows(size).enumerate() {
        if w.iter().all_unique() {
            return i + size;
        }
    }

    unreachable!();
}
