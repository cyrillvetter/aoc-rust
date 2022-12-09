use std::collections::HashSet;
use itertools::Itertools;
use crate::solution::Solution;

const ADJACENT_OVERLAP: [(i32, i32); 9] = [(0, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

pub fn part_one(input: &str) -> Solution {
    let motions = parse(input);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(tail_pos);

    for motion in motions {
        for _ in 0..motion.steps {
            let prev_head = head_pos;
            head_pos.0 += motion.direction.0;
            head_pos.1 += motion.direction.1;
            if !is_adjacent(tail_pos, head_pos) {
                tail_pos = prev_head;
            }
            visited.insert(tail_pos);
        }
    }

    Solution::USize(visited.len())
}

pub fn part_two(input: &str) -> Solution {
    let motions = parse(input);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut positions: [(i32, i32); 10] = [(0, 0); 10];
    visited.insert(positions[9]);

    for motion in motions {
        for _ in 0..motion.steps {
            positions[0].0 += motion.direction.0;
            positions[0].1 += motion.direction.1;
            let mut prev = positions[0];

            for i in 1..positions.len() {
                let curr = positions[i];
                if !is_adjacent(curr, positions[i - 1]) {
                    positions[i].0 += calc_next_pos(prev.0, curr.0);
                    positions[i].1 += calc_next_pos(prev.1, curr.1);
                }

                prev = positions[i];
            }

            visited.insert(positions[9]);
        }
    }

    Solution::USize(visited.len())
}

struct Motion {
    direction: (i32, i32),
    steps: u32,
}

fn parse(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|t| Motion {
                direction: match t.0 {
                    "R" => (0, 1),
                    "L" => (0, -1),
                    "U" => (1, 0),
                    "D" => (-1, 0),
                    _ => unreachable!(),
                },
                steps: t.1.parse().unwrap()
            })
        .collect_vec()
}

fn is_adjacent(tail: (i32, i32), head: (i32, i32)) -> bool {
    ADJACENT_OVERLAP.iter().any(|(x, y)| (tail.0 + x, tail.1 + y) == head)
}

fn calc_next_pos(x: i32, y: i32) -> i32 {
    let mut new_p = x - y;
    if new_p > 1 {
        new_p -= 1;
    } else if new_p < -1 {
        new_p += 1;
    }

    new_p
}
