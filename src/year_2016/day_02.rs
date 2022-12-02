use crate::solution::Solution;
use std::cmp::{max, min};

pub fn part_one(input: &str) -> Solution {
    let mut code = String::new();

    let keypad = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let mut curr: (i32, i32) = (1, 1);

    for lines in input.lines() {
        let mut button = 5;
        for instr in lines.chars() {
            let m = instr_mut(instr);
            curr = (max(0, min(2, curr.0 + m.0)), max(0, min(2, curr.1 + m.1)));
            button = keypad[curr.0 as usize][curr.1 as usize];
        }

        code.push_str(&button.to_string())
    }

    Solution::Str(code)
}

pub fn part_two(input: &str) -> Solution {
    let mut code = String::new();
    let mut button = '5';
    let mut curr: (i32, i32) = (2, 0);

    let keypad = vec![
        vec!['.', '.', '1', '.', '.'],
        vec!['.', '2', '3', '4', '.'],
        vec!['5', '6', '7', '8', '9'],
        vec!['.', 'A', 'B', 'C', '.'],
        vec!['.', '.', 'D', '.', '.'],
    ];

    for lines in input.lines() {
        for instr in lines.chars() {
            let m = instr_mut(instr);
            let new = (max(0, min(4, curr.0 + m.0)), max(0, min(4, curr.1 + m.1)));
            let next = keypad[new.0 as usize][new.1 as usize];
            if next != '.' {
                curr = new;
                button = next;
            }
        }

        code.push_str(&button.to_string())
    }

    Solution::Str(code)
}

fn instr_mut(instr: char) -> (i32, i32) {
    match instr {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2016, 2);
        assert_eq!(part_one(&input), Solution::Str(String::from("99332")));
        assert_eq!(part_two(&input), Solution::Str(String::from("DD483")));
    }
}