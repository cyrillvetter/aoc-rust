use crate::common::read_string;
use itertools::Itertools;

pub fn part_one() -> String {
    read_string(2022, 1)
        .split("\n\n")
        .into_iter()
        .map(|v| v.split("\n").into_iter().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .max().unwrap()
        .to_string()
}

pub fn part_two() -> String {
    read_string(2022, 1)
        .split("\n\n")
        .into_iter()
        .map(|v| v.split("\n").into_iter().map(|i| i.parse::<i32>().unwrap()).sum::<i32>())
        .sorted_by(|a, b| Ord::cmp(&b, &a)) // Sort in descending order
        .take(3)
        .sum::<i32>()
        .to_string()
}
