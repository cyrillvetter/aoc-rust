use std::collections::HashSet;
use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let tree_heights = input.lines().map(|l| l.as_bytes().iter().cloned().collect_vec()).collect_vec();

    for (y, l) in tree_heights.iter().enumerate() {
        let mut tallest: u8 = 0;

        for (x, height) in l.iter().enumerate() {
            check_visibility(&mut visible_trees, &mut tallest, *height, (y, x));
        }

        tallest = 0;

        for (x, height) in l.iter().enumerate().rev() {
            check_visibility(&mut visible_trees, &mut tallest, *height, (y, x));
        }
    }

    for x in 0..tree_heights[0].len() {
        let mut tallest: u8 = 0;

        for (y, line) in tree_heights.iter().enumerate() {
            check_visibility(&mut visible_trees, &mut tallest, line[x], (y, x));
        }

        tallest = 0;

        for (y, line) in tree_heights.iter().enumerate().rev() {
            check_visibility(&mut visible_trees, &mut tallest, line[x], (y, x));
        }
    }

    Solution::USize(visible_trees.len())
}

pub fn part_two(input: &str) -> Solution {
    let tree_heights: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
    let mut highest_score = 0;

    for (y, l) in tree_heights.iter().enumerate() {
        for x in 0..l.len() {
            let score_left = get_directional_score(&tree_heights, x, y, (0, -1));
            let score_right = get_directional_score(&tree_heights, x, y, (0, 1));
            let score_up = get_directional_score(&tree_heights, x, y, (-1, 0));
            let score_down = get_directional_score(&tree_heights, x, y, (1, 0));

            let score = score_left * score_right * score_up * score_down;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    Solution::U32(highest_score)
}

fn check_visibility(visible_trees: &mut HashSet<(usize, usize)>, max_size: &mut u8, height: u8, location: (usize, usize)) {
    if height > *max_size {
        *max_size = height;
        visible_trees.insert(location);
    }
}

fn get_directional_score(trees: &Vec<Vec<u32>>, x: usize, y: usize, direction: (isize, isize)) -> u32 {
    let tree_height = trees[y][x];

    let y_boundary = trees.len() as isize - 1;
    let x_boundary = trees[0].len() as isize - 1;

    let mut x = x as isize;
    let mut y = y as isize;

    let mut score = 0;
    while x > 0 && y > 0 && x < x_boundary && y < y_boundary {
        y += direction.0;
        x += direction.1;

        let next_height = trees[y as usize][x as usize];
        if next_height >= tree_height {
            return score + 1;
        }

        score += 1;
    }

    score
}
