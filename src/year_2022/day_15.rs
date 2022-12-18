use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};

pub fn part_one(input: &str) -> Solution {
    const ROW: i32 = 2000000;
    let pairs = parse(input);
    let mut ranges: Vec<(i32, i32)> = Vec::new();

    for p in pairs {
        let max_dist = manhattan_dist(&p.sensor, &p.beacon) as i32;
        let row_dist = p.sensor.y.abs_diff(ROW) as i32;
        let diff = max_dist - row_dist;
        if diff > 0 {
            ranges.push((p.sensor.x - diff, p.sensor.x + diff));
        }
    }

    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut no_beacons_pos: u32 = 0;
    let mut end_prev = i32::MIN;

    for r in ranges.iter() {
        let mut start = r.0;
        let end = r.1;

        if end <= end_prev {
            continue;
        }

        if start <= end_prev {
            start = end_prev;
        }

        no_beacons_pos += end.abs_diff(start);
        end_prev = end;
    }

    Solution::U32(no_beacons_pos)
}

pub fn part_two(input: &str) -> Solution {
    const LOWER_LIMIT: i32 = 0;
    const UPPER_LIMIT: i32 = 4_000_000;
    const FREQUENCY_TUNER: u64 = 4_000_000;

    let pairs = parse(input);

    for row in (0..=UPPER_LIMIT).rev() {
        let mut ranges: Vec<(i32, i32)> = Vec::new();

        for p in &pairs {
            let max_dist = manhattan_dist(&p.sensor, &p.beacon) as i32;
            let row_dist = p.sensor.y.abs_diff(row) as i32;
            let diff = max_dist - row_dist;
            if diff > 0 {
                let mut start = max(p.sensor.x - diff, LOWER_LIMIT);
                let mut end = min(p.sensor.x + diff, UPPER_LIMIT);

                ranges.push((start, end));
            }
        }

        ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut end_prev = LOWER_LIMIT;

        for r in ranges.iter() {
            let mut start = r.0;
            let end = r.1;

            if start > end_prev + 1 {
                let tuning_frequency = (end_prev + 1) as u64 * FREQUENCY_TUNER + row as u64;
                return Solution::U64(tuning_frequency);
            }

            end_prev = max(end_prev, end);
        }
    }

    unreachable!()
}

struct Pair {
    sensor: Point,
    beacon: Point,
}

impl Pair {
    fn new(sensor_x: i32, sensor_y: i32, beacon_x: i32, beacon_y: i32) -> Self {
        Self { sensor: Point { x: sensor_x, y: sensor_y }, beacon: Point { x: beacon_x, y: beacon_y } }
    }
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
        .map(|c| Pair::new(c[0], c[1], c[2], c[3]))
        .collect_vec()
}
