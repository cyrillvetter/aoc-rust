use crate::solution::Solution;

pub fn part_one(input: &str) -> Solution {
    let result = input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>();

    Solution::I32(result)
}

pub fn part_two(input: &str) -> Solution {
    let mut floor: i32 = 0;
    for (i, n) in input.chars().map(|c| if c == '(' { 1i32 } else { -1i32 }).enumerate() {
        floor += n;

        if floor < 0 {
            return Solution::USize(i + 1);
        }
    };

    unreachable!();
}
