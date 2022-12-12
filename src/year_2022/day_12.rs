use crate::solution::Solution;
use std::collections::VecDeque;
use itertools::Itertools;

const ADJACENT: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part_one(input: &str) -> Solution {
    let start = get_position_of(&input, 'S');
    let end = get_position_of(&input, 'E');
    let grid = parse(input);
    println!("{:?}", grid);
    let steps = bfs(grid, start, end).expect("Path not found");

    Solution::USize(steps)
}

pub fn part_two(input: &str) -> Solution {
    Solution::Empty
}

fn bfs(grid: Vec<Vec<u8>>, source: (i32, i32), target: (i32, i32)) -> Option<usize> {
    let mut visited = vec![vec![None::<(i32, i32)>; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    queue.push_front((source.0, source.1));
    visited[source.0 as usize][source.1 as usize] = Some(source);

    while let Some((y, x)) = queue.pop_front() {
        let curr_height = grid[y as usize][x as usize];

        /*println!("\nExpanding ({}, {})", x, y);
        for iy in 0..visited.len() {
            for ix in 0..visited[0].len() {
                if y as usize == iy && x as usize == ix {
                    print!("#");
                } else if (iy as i32, ix as i32) == target && visited[iy][ix].is_some() {
                    print!("X");
                } else if visited[iy][ix].is_some() {
                    print!("0");
                } else {
                    print!("{}", grid[iy][ix] as char);
                }
            }
            println!();
        }*/

        if (y, x) == target {
            println!("FOUND");
            let mut steps = 0;

            let mut prev_x = x;
            let mut prev_y = y;
            while prev_x != source.1 || prev_y != source.0 {
                let (py, px) = visited[prev_y as usize][prev_x as usize].unwrap();
                //println!("{}", grid[py as usize][px as usize] as char);
                steps += 1;
                prev_y = py;
                prev_x = px;
            }

            return Some(steps);
        }

        for (ay, ax) in &ADJACENT {
            let adj_y = (y + ay) as usize;
            let adj_x = (x + ax) as usize;

            if x + ax < 0 || y + ay < 0 || adj_y >= grid.len() || adj_x >= grid[0].len() {
                println!("out of bounds");
                continue;
            }

            let adj_height = grid[adj_y][adj_x];
            if adj_height < curr_height || adj_height > curr_height + 1 {
                println!("step from {:?} to {:?} not possible", curr_height as char, adj_height as char);
                continue;
            }

            if visited[adj_y][adj_x].is_some() {
                println!("y {}, x {} already visited", adj_y, adj_x);
                continue;
            }

            visited[adj_y][adj_x] = Some((y, x));
            queue.push_back((y + ay, x + ax));
        }

        println!("queued: {}", queue.len());
        println!();
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

fn get_position_of(grid: &str, of: char) -> (i32, i32) {
    for (y, i) in grid.lines().enumerate() {
        if i.contains(of) {
            let x = i.chars().position(|c| c == of).unwrap();
            return (y as i32, x as i32);
        }
    }

    panic!()
}
