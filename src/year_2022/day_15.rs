use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;

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
    const LOWER_LIMIT: u32 = 0;
    const UPPER_LIMIT: u32 = 20;

    let pairs = parse(input);

    Solution::Empty
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
