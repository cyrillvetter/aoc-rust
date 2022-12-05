use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instructions = build_instructions(l, &number_match);
        for _ in 0..instructions.0 {
            let item = crates[instructions.1 - 1].pop().unwrap();
            crates[instructions.2 - 1].push(item)
        }
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

pub fn part_two(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instructions = build_instructions(l, &number_match);

        for i in (0..instructions.0).rev() {
            let len = crates[instructions.1 - 1].len() - 1 - i;
            let item = crates[instructions.1 - 1].remove(len);
            crates[instructions.2 - 1].push(item);
        }
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

fn build_crates(crate_part: &str) -> Vec<Vec<char>> {
    let input_stack = crate_part.lines().rev().collect_vec();
    let mut crates: Vec<Vec<char>> = Vec::new();

    for (i, c) in input_stack[0].chars().enumerate() {
        if c.is_whitespace() {
            continue;
        }

        let mut stack: Vec<char> = Vec::new();
        for l in input_stack.iter().skip(1) {
            if let Some(crate_char) = l.chars().nth(i) {
                if !crate_char.is_whitespace() {
                    stack.push(crate_char);
                }
            }
        }

        crates.push(stack);
    }

    crates
}

fn build_instructions(line: &str, pattern: &Regex) -> (usize, usize, usize) {
    pattern
        .find_iter(line)
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect_tuple::<(_, _, _)>().unwrap()
}
