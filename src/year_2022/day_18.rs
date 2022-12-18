use crate::solution::Solution;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Solution {
    let sides_count = input.lines().count() * 6;
    let unique_sides: HashSet<Point> = input
        .lines()
        .map(|l| l.split(',').map(|c| c.parse::<i32>().unwrap()).collect_vec())
        .flat_map(|t| Point::new(t[0] * 2, t[1] * 2, t[2] * 2).adj())
        .collect();

    Solution::USize(sides_count - (sides_count - unique_sides.len()) * 2)
}

pub fn part_two(input: &str) -> Solution {
    let mut sides = input
        .lines()
        .map(|l| l.split(',').map(|c| c.parse::<i32>().unwrap()).collect_vec())
        .map(|t| Point::new(t[0], t[1], t[2]))
        .collect();

    Solution::USize(count_exterior_sides(&sides))
}

// Flood-fill algorithm
fn count_exterior_sides(cubes: &HashSet<Point>) -> usize {
    // Flood boundaries
    let max_x = cubes.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = cubes.iter().map(|p| p.y).max().unwrap() + 1;
    let max_z = cubes.iter().map(|p| p.z).max().unwrap() + 1;

    let min_x = cubes.iter().map(|p| p.x).min().unwrap() - 1;
    let min_y = cubes.iter().map(|p| p.y).min().unwrap() - 1;
    let min_z = cubes.iter().map(|p| p.z).min().unwrap() - 1;

    let mut exterior = 0;
    let start = Point { x: min_x, y: min_y, z: min_z };

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(start.clone());
    visited.insert(start);

    while let Some(p) = queue.pop_front() {
        for c in p.adj() {
            if cubes.contains(&c) {
                exterior += 1;
            } else if !visited.contains(&c) &&
                min_x <= c.x &&
                min_y <= c.y &&
                min_z <= c.z &&
                max_x >= c.x &&
                max_y >= c.y &&
                max_z >= c.z {
                queue.push_back(c.clone());
                visited.insert(c);
            }
        }
    }

    exterior
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn offset(&self, o: (i32, i32, i32)) -> Self {
        Point::new(self.x + o.0, self.y + o.1, self.z + o.2)
    }

    fn adj(&self) -> [Point; 6] {
        [
            self.offset((1, 0, 0)),
            self.offset((-1, 0, 0)),
            self.offset((0, 1, 0)),
            self.offset((0, -1, 0)),
            self.offset((0, 0, 1)),
            self.offset((0, 0, -1)),
        ]
    }
}
