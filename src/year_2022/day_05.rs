use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instr = build_instructions(l, &number_match);

        let remove_index = crates[instr.from - 1].len() - instr.amount;
        let mut pulled_crates = crates[instr.from - 1].split_off(remove_index);

        pulled_crates.reverse();
        crates[instr.to - 1].append(&mut pulled_crates);
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

pub fn part_two(input: &str) -> Solution {
    let parts = input.split_once("\n\n").unwrap();
    let number_match = Regex::new("\\d+").unwrap();
    let mut crates = build_crates(parts.0);

    for l in parts.1.lines() {
        let instr = build_instructions(l, &number_match);

        let remove_index = crates[instr.from - 1].len() - instr.amount;
        let mut pulled_crates = crates[instr.from - 1].split_off(remove_index);
        crates[instr.to - 1].append(&mut pulled_crates);
    }

    let top_crates: String = crates.iter().map(|s| s.last().unwrap()).collect();
    Solution::Str(top_crates)
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
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

fn build_instructions(line: &str, pattern: &Regex) -> Instruction {
    let tup = pattern
        .find_iter(line)
        .map(|c| c.as_str().parse::<usize>().unwrap())
        .collect_vec();

    Instruction {
        amount: tup[0],
        from: tup[1],
        to: tup[2],
    }
}
