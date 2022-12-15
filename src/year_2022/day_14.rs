use crate::solution::Solution;
use itertools::Itertools;
use std::cmp::{max, min};

const SAND_SOURCE: (usize, usize) = (0, 500);
const DIAG_DOWN_ADJ: [(usize, isize); 2] = [(1, -1), (1, 1)];

pub fn part_one(input: &str) -> Solution {
    let rocks = parse(input);
    let (y_max, x_max) = get_max_rocks_coords(&rocks);
    let mut cave = vec![vec![false; x_max * 2]; y_max * 2];

    draw_rocks(&mut cave, &rocks);

    let mut sand_units: u32 = 0;

    'outer: loop {
        let mut curr = SAND_SOURCE;
        loop {
            let prev = curr;
            curr = match straight_drop(&cave, curr, y_max) {
                Some(y) => (y, curr.1),
                None => break 'outer, // Sand overflows to infinity
            };
            curr = diagonal_drop(&cave, curr);

            if curr == prev {
                break;
            }
        }

        cave[curr.0][curr.1] = true;
        sand_units += 1;
    }

    Solution::U32(sand_units)
}

pub fn part_two(input: &str) -> Solution {
    let rocks = parse(input);
    let (y_max, x_max) = get_max_rocks_coords(&rocks);

    let mut cave = vec![vec![false; x_max * 2]; y_max * 2];

    draw_rocks(&mut cave, &rocks);

    // Draw floor
    for i in 0..cave[y_max + 2].len() {
        cave[y_max + 2][i] = true;
    }

    let mut sand_units: u32 = 0;

    'outer: loop {
        let mut curr = SAND_SOURCE;
        sand_units += 1;
        loop {
            let prev = curr;
            curr = match straight_drop(&cave, curr, y_max + 3) {
                Some(y) => (y, curr.1),
                _ => unreachable!(),
            };
            curr = diagonal_drop(&cave, curr);

            // Sand source is blocked
            if curr == SAND_SOURCE {
                break 'outer;
            }

            if curr == prev {
                break;
            }
        }

        cave[curr.0][curr.1] = true;
    }

    Solution::U32(sand_units)
}

fn straight_drop(cave: &[Vec<bool>], coord: (usize, usize), y_max: usize) -> Option<usize> {
    let mut y_coord = coord.0 + 1;
    while !cave[y_coord][coord.1] {
        y_coord += 1;
        if y_coord > y_max {
            return None;
        }
    }

    Some(y_coord - 1)
}

fn diagonal_drop(cave: &[Vec<bool>], coord: (usize, usize)) -> (usize, usize) {
    for (off_y, off_x) in DIAG_DOWN_ADJ.iter() {
        let fall_y = coord.0 + off_y;
        let fall_x = (coord.1 as isize + off_x) as usize;

        if !cave[fall_y][fall_x] {
            return (fall_y, fall_x);
        }
    }

    coord
}

fn draw_rocks(cave: &mut [Vec<bool>], rocks: &Vec<Vec<(usize, usize)>>) {
    for r in rocks {
        for w in r.windows(2) {
            let from = w[0];
            let to = w[1];
            for y in min(from.0, to.0)..=(max(from.0, to.0)) {
                for x in min(from.1, to.1)..=(max(from.1, to.1)) {
                    cave[y][x] = true;
                }
            }
        }
    }
}

fn get_max_rocks_coords(rocks: &[Vec<(usize, usize)>]) -> (usize, usize) {
    let y_max = rocks.iter().map(|r| r.iter().map(|i| i.0).max().unwrap()).max().unwrap();
    let x_max = rocks.iter().map(|r| r.iter().map(|i| i.1).max().unwrap()).max().unwrap();

    (y_max, x_max)
}

fn parse(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|l| l.split(" -> ").map(|s| s.split_once(',').unwrap()))
        .map(|t| t.map(|it| (it.1.parse::<usize>().unwrap(), it.0.parse::<usize>().unwrap())).collect_vec())
        .collect_vec()
}
