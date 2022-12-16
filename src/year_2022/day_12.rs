use crate::solution::Solution;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use itertools::Itertools;

const ADJACENT: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part_one(input: &str) -> Solution {
    let start = get_first_position_of(input, b'E');
    let grid = parse(input);
    let steps = bfs(&grid, start, b'S').unwrap();

    Solution::USize(steps)
}

pub fn part_two(input: &str) -> Solution {
    let start = get_first_position_of(input, b'E');
    let grid = parse(input);
    let steps = bfs(&grid, start, b'a').unwrap();

    Solution::USize(steps)
}

fn bfs(grid: &Vec<Vec<u8>>, source: (usize, usize), target: u8) -> Option<usize> {
    let cols = grid.len();
    let rows = grid[0].len();

    let mut visited = vec![vec![None::<(usize, usize)>; rows]; cols];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_front((source.0, source.1));
    visited[source.0 as usize][source.1 as usize] = Some(source);

    while let Some((y, x)) = queue.pop_front() {
        let curr_height = grid[y][x];

        if curr_height == target {
            let mut steps = 0;
            let mut prev_y = y;
            let mut prev_x = x;

            while prev_y != source.0 || prev_x != source.1 {
                let (py, px) = visited[prev_y][prev_x].unwrap();

                steps += 1;
                prev_y = py;
                prev_x = px;
            }

            return Some(steps);
        }

        for (ay, ax) in &ADJACENT {
            let signed_adj_y = y as isize + ay;
            let adj_y = signed_adj_y as usize;
            let signed_adj_x = x as isize + ax;
            let adj_x = signed_adj_x as usize;

            if signed_adj_x < 0 || signed_adj_y < 0 || adj_y >= cols || adj_x >= rows {
                continue;
            }

            if get_char_height(grid[adj_y][adj_x]) + 1 < get_char_height(curr_height) {
                continue;
            }

            if visited[adj_y][adj_x].is_some() {
                continue;
            }

            visited[adj_y][adj_x] = Some((y, x));
            queue.push_back((adj_y, adj_x));
        }
    }

    None
}

fn get_char_height(char: u8) -> u8 {
    match char {
        b'S' => b'a',
        b'E' => b'z',
        _ => char
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect_vec()
}

fn get_first_position_of(grid: &str, of: u8) -> (usize, usize) {
    grid
        .lines()
        .enumerate()
        .find_map(|(y, l)| l.as_bytes().iter().position(|c| *c == of).map(|x| (y, x)))
        .unwrap()
}
