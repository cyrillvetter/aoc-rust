use crate::solution::Solution;
use std::collections::VecDeque;

const ADJACENT: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn part_one(input: &str) -> Solution {
    let start = get_index_of(&input, 'S');
    let end = get_index_of(&input, 'E');
    let grid = parse(input);
    let steps = bfs(grid, start, end).unwrap();

    Solution::USize(steps)
}

pub fn part_two(input: &str) -> Solution {
    Solution::Empty
}

fn bfs(grid: Vec<Vec<u8>>, source: (i32, i32), target: (i32, i32)) -> Option<usize> {
    let mut visited = vec![vec![None::<(i32, i32)>; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    queue.push_back((source.0, source.1));
    visited[source.0 as usize][source.1 as usize] = Some(source);

    while let Some((y, x)) = queue.pop_front() {
        let curr_height = grid[y as usize][x as usize];

        if (y, x) == target {
            println!("FOUND");
            let mut steps = 0;

            let mut prev_x = x;
            let mut prev_y = y;
            while prev_x != 0 || prev_y != 0 {
                let (py, px) = visited[prev_y as usize][prev_x as usize].unwrap();
                steps += 1;
                prev_y = py;
                prev_x = px;
            }

            return Some(steps);
        }

        for (ax, ay) in &ADJACENT {
            let adj_y = (y + ay) as usize;
            let adj_x = (x + ax) as usize;

            if x + ax < 0 || y + ay < 0 || adj_y >= grid.len() || adj_x >= grid[adj_y].len() {
                continue;
            }

            let adj_height = grid[adj_y][adj_x];
            if adj_height < curr_height || adj_height > curr_height + 1 {
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
            if c == 83 {
                c = 97;
            } else if c == 69 {
                c = 122;
            }

            points.push(c)
        }

        grid.push(points);
    }

    grid
}

fn get_index_of(grid: &str, of: char) -> (i32, i32) {
    for (y, i) in grid.lines().enumerate() {
        if i.contains(of) {
            let x = i.chars().position(|c| c == of).unwrap();
            return (y as i32, x as i32);
        }
    }

    panic!()
}
