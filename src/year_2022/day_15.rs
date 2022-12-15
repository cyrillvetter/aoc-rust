use std::collections::HashSet;
use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Solution {
    const ROW: i32 = 2000000;
    let pairs = parse(input);
    let mut no_beacon_pos: HashSet<i32> = HashSet::new();

    for p in pairs {
        let max_dist = manhattan_dist(&p.sensor, &p.beacon) as i32;
        let row_dist = p.sensor.y.abs_diff(ROW) as i32;
        let diff = max_dist - row_dist;
        if diff > 0 {
            for row_x in p.sensor.x - diff..=p.sensor.x + diff {
                no_beacon_pos.insert(row_x);
            }

            if p.beacon.y == ROW {
                no_beacon_pos.remove(&p.beacon.x);
            }
        }
    }

    Solution::USize(no_beacon_pos.len())
}

pub fn part_two(input: &str) -> Solution {
    const LOWER_LIMIT: u32 = 0;
    const UPPER_LIMIT: u32 = 20;

    let pairs = parse(input);

    Solution::Empty
}

struct Pair {
    sensor: Point,
    beacon: Point,
}

struct Point {
    x: i32,
    y: i32,
}

fn manhattan_dist(a: &Point, b: &Point) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn parse(input: &str) -> Vec<Pair> {
    let signed_nums_pattern = Regex::new("-?\\d+").unwrap();

    input
        .lines()
        .map(|l| signed_nums_pattern
            .find_iter(l)
            .map(|r| r.as_str().parse::<i32>().unwrap())
            .collect_vec())
        .map(|c| Pair { sensor: Point { x: c[0], y: c[1] }, beacon: Point { x: c[2], y: c[3] } })
        .collect_vec()
}
