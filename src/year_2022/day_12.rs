use crate::solution::Solution;
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part_one(input: &str) -> Solution {
    let start = get_position_of(&input, 'S');
    let end = get_position_of(&input, 'E');
    let grid = parse(input);
    let steps = get_shortest_path(&grid, start, end).unwrap();

    Solution::USize(steps)
}

pub fn part_two(input: &str) -> Solution {
    let end = get_position_of(&input, 'E');
    let grid = parse(input);

    let mut shortest_path = usize::MAX;
    for (y, l) in grid.iter().enumerate() {
        for (x, _) in l.iter().enumerate().filter(|(_, c)| **c == b'a') {
            if let Some(steps) = get_shortest_path(&grid, (y as i32, x as i32), end) {
                if steps < shortest_path {
                    shortest_path = steps;
                }
            }
        }
    }

    Solution::USize(shortest_path)
}

// Breadth-first search
fn get_shortest_path(grid: &Vec<Vec<u8>>, source: (i32, i32), target: (i32, i32)) -> Option<usize> {
    let cols = grid.len();
    let rows = grid[0].len();

    let mut visited = vec![vec![None::<(i32, i32)>; rows]; cols];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    queue.push_front((source.0, source.1));
    visited[source.0 as usize][source.1 as usize] = Some(source);

    while let Some((y, x)) = queue.pop_front() {
        let curr_height = grid[y as usize][x as usize];

        if (y, x) == target {
            let mut steps = 0;

            let mut prev_y = y;
            let mut prev_x = x;
            while prev_y != source.0 || prev_x != source.1 {
                let (py, px) = visited[prev_y as usize][prev_x as usize].unwrap();

                steps += 1;
                prev_y = py;
                prev_x = px;
            }

            return Some(steps);
        }

        for (ay, ax) in &ADJACENT {
            let adj_y = (y + ay) as usize;
            let adj_x = (x + ax) as usize;

            if x + ax < 0 || y + ay < 0 || adj_y >= cols || adj_x >= rows {
                continue;
            }

            if grid[adj_y][adj_x] - 1 > curr_height {
                continue;
            }

            if visited[adj_y][adj_x].is_some() {
                continue;
            }

            visited[adj_y][adj_x] = Some((y, x));
            queue.push_back((y + ay, x + ax));
        }
    }

    None
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for l in input.lines() {
        let mut points: Vec<u8> = Vec::new();
        for c in l.as_bytes().iter() {
            let mut c = *c;

            if c == b'S' {
                c = b'a';
            } else if c == b'E' {
                c = b'z';
            }

            points.push(c)
        }

        grid.push(points);
    }

    grid
}

fn get_position_of(grid: &str, of: char) -> (i32, i32) {
    for (y, i) in grid.lines().enumerate() {
        if i.contains(of) {
            let x = i.chars().position(|c| c == of).unwrap();
            return (y as i32, x as i32);
        }
    }

    unreachable!();
}
