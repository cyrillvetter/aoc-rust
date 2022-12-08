use crate::solution::Solution;
use itertools::Itertools;

pub fn part_one(input: &str) -> Solution {
    let mut visible_trees = 0;
    let forest = input.lines().map(|l| l.as_bytes().iter().cloned().collect_vec()).collect_vec();
    let y_boundary = forest.len();
    let x_boundary = forest[0].len();

    for (y, l) in forest.iter().enumerate().skip(1).take(forest.len() - 2) {
        for (x, height) in l.iter().enumerate().skip(1).take(l.len() - 2) {
            if !l[0..x].iter().any(|t| t >= height) ||
                !l[x+1..x_boundary].iter().any(|t| t >= height) ||
                !forest[0..y].iter().map(|i| i[x]).any(|t| t >= *height) ||
                !forest[y+1..y_boundary].iter().map(|i| i[x]).any(|t| t >= *height) {
                visible_trees += 1;
            }
        }
    }

    Solution::USize(visible_trees + 2 * x_boundary + 2 * y_boundary - 4)
}

pub fn part_two(input: &str) -> Solution {
    let forest: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
    let y_boundary = forest.len();
    let x_boundary = forest[0].len();
    let mut highest_score = 0;

    for (y, l) in forest.iter().enumerate().skip(1).take(forest.len() - 2) {
        for (x, height) in l.iter().enumerate().skip(1).take(l.len() - 2) {
            let score_left = l[0..x].iter().rev().take(x - 1).take_while(|t| height > *t).count() + 1;
            let score_right = l[x+1..x_boundary].iter().skip(1).take_while(|t| height > *t).count() + 1;
            let score_up = forest[0..y].iter().rev().take(y - 1).map(|i| i[x]).take_while(|t| height > t).count() + 1;
            let score_down = forest[y+1..y_boundary].iter().skip(1).map(|i| i[x]).take_while(|t| height > t).count() + 1;

            let score = score_left * score_right * score_up * score_down;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    Solution::USize(highest_score)
}
