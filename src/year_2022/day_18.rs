use std::collections::HashSet;
use crate::solution::Solution;
use itertools::Itertools;

const SIDES: [(i32, i32, i32); 6] = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

pub fn part_one(input: &str) -> Solution {
    let sides_count = input.lines().count() * 6;
    let unique_sides = input
        .lines()
        .map(|l| l.split(',').map(|c| c.parse::<i32>().unwrap()).collect_vec())
        .flat_map(|t| Point { x: t[0] * 2, y: t[1] * 2, z: t[2] * 2 }.sides())
        .unique()
        .count();

    Solution::USize(sides_count - (sides_count - unique_sides) * 2)
}

pub fn part_two(input: &str) -> Solution {
    Solution::Empty
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn sides(&self) -> Vec<Point> {
        SIDES
            .iter()
            .map(|s| Point { x: self.x + s.0, y: self.y + s.1, z: self.z + s.2 })
            .collect()
    }
}
