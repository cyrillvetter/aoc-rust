use crate::solution::Solution;
use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Solution {
    const ROUNDS: u16 = 20;
    let monkeys = parse(input);

    Solution::U64(solve(monkeys, ROUNDS, &|i| i / 3))
}

pub fn part_two(input: &str) -> Solution {
    const ROUNDS: u16 = 10_000;
    let monkeys = parse(input);
    let modulo: u64 = monkeys.iter().map(|m| m.divide).product();

    Solution::U64(solve(monkeys, ROUNDS, &|i| i % modulo))
}

fn solve(mut monkeys: Vec<Monkey>, rounds: u16, reduce_worry_fn: &dyn Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let mut true_throw: Vec<u64> = Vec::new();
            let mut false_throw: Vec<u64> = Vec::new();

            for item in &m.items {
                let mut item_worry = *item;
                let operand = m.operation.unwrap_or(item_worry);
                item_worry = if m.is_operation_addition { item_worry + operand } else { item_worry * operand };

                item_worry = reduce_worry_fn(item_worry);

                if item_worry % m.divide == 0 {
                    true_throw.push(item_worry);
                } else {
                    false_throw.push(item_worry);
                }

                m.inspections += 1;
            }

            m.items.clear();

            let dividable_throw_to = m.throw_to_if_dividable;
            let not_dividable_throw_to = m.throw_to_if_not_dividable;

            monkeys[dividable_throw_to].items.extend(true_throw);
            monkeys[not_dividable_throw_to].items.extend(false_throw);
        }
    }

    monkeys.iter().map(|m| m.inspections).sorted().rev().take(2).product()
}

struct Monkey {
    items: Vec<u64>,
    inspections: u64,
    operation: Option<u64>,
    is_operation_addition: bool,
    divide: u64,
    throw_to_if_dividable: usize,
    throw_to_if_not_dividable: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    let numbers_pattern = Regex::new("\\d+").unwrap();
    let mut monkeys = Vec::new();

    for m in input.split("\n\n") {
        let l = m.lines().skip(1).collect_vec();

        let items = numbers_pattern.find_iter(l[0]).map(|c| c.as_str().parse::<u64>().unwrap()).collect_vec();

        let is_operation_addition = l[1].contains('+');
        let operation = l[1].rsplit(' ').next().unwrap().parse::<u64>().ok();

        let divide = numbers_pattern.find(l[2]).unwrap().as_str().parse::<u64>().unwrap();
        let throw_to_if_dividable = numbers_pattern.find(l[3]).unwrap().as_str().parse::<usize>().unwrap();
        let throw_to_if_not_dividable = numbers_pattern.find(l[4]).unwrap().as_str().parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            inspections: 0,
            operation,
            is_operation_addition,
            divide,
            throw_to_if_dividable,
            throw_to_if_not_dividable
        })
    }

    monkeys
}
