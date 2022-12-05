use itertools::Itertools;
use crate::solution::Solution;

pub fn part_one(input: &str) -> Solution {
    let result = input
        .lines()
        .map(|p| p.split('x').map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .map(|mut s| {
            let result = 2 * s[0] * s[1] + 2 * s[1] * s[2] + 2 * s[2] * s[0];
            s.sort();
            result + s[0] * s[1]
        })
        .sum::<i32>();

    Solution::I32(result)
}

pub fn part_two(input: &str) -> Solution {
    let result = input
        .lines()
        .map(|p| p.split('x').map(|s| s.parse().unwrap()).sorted().collect::<Vec<i32>>())
        .map(|s| 2 * s[0] + 2 * s[1] + s[0] * s[1] * s[2])
        .sum::<i32>();

    Solution::I32(result)
}
