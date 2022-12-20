use std::collections::HashSet;
use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    const TIMES: usize = 1;

    let mut nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .enumerate()
        .collect_vec();

    let result = decrypt(&mut nums, TIMES);
    Solution::I64(result)
}

pub fn part_two(input: &str) -> Solution {
    const DECRYPTION_KEY: i64 = 811_589_153;
    const TIMES: usize = 10;

    let mut nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(|n| n * DECRYPTION_KEY)
        .enumerate()
        .collect_vec();

    let result = decrypt(&mut nums, TIMES);
    Solution::I64(result)
}

fn decrypt(nums: &mut Vec<(usize, i64)>, times: usize) -> i64 {
    let initial = nums.clone();
    let size = nums.len();

    for _ in 0..times {
        for i in 0..size {
            let curr = initial[i];
            let pos = nums.iter().position(|n| *n == curr).unwrap();
            let next = wrap(curr.1 + pos as i64, size as i64);

            nums.remove(pos);
            nums.insert(next, curr);
        }
    }

    let zero_pos = nums.iter().position(|n| (*n).1 == 0).unwrap();

    let num_1000 = nums[(zero_pos + 1000) % size];
    let num_2000 = nums[(zero_pos + 2000) % size];
    let num_3000 = nums[(zero_pos + 3000) % size];

    num_1000.1 + num_2000.1 + num_3000.1
}

fn wrap(index: i64, mut size: i64) -> usize {
    size -= 1;
    (((index % size) + size) % size) as usize
}
