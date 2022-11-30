use crate::common::read_lines;
use regex::Regex;

pub fn part_one() -> String {
    let lines = read_lines(2015, 6);
    let mut grid = vec![vec![false; 1000]; 1000];

    let instr_regex = Regex::new("(turn off|turn on|toggle)\\s(\\d+),(\\d+)\\sthrough\\s(\\d+),(\\d+)").unwrap();
    let mut counter = 0;

    for l in lines {
        let capts = instr_regex.captures(&l).unwrap();

        let action = &capts[1];
        let from_x = capts[2].parse::<usize>().unwrap();
        let from_y = capts[3].parse::<usize>().unwrap();
        let to_x = capts[4].parse::<usize>().unwrap();
        let to_y = capts[5].parse::<usize>().unwrap();

        for y in grid.iter_mut().skip(from_y).take(to_y - from_y + 1) {
            for x in y.iter_mut().skip(from_x).take(to_x - from_x + 1) {
                let prev = x.clone();

                *x = match action {
                    "turn on" => true,
                    "turn off" => false,
                    _ => !(*x),
                };

                if !prev && *x {
                    counter += 1;
                } else if prev && !*x {
                    counter -= 1;
                }
            }
        }
    }

    counter.to_string()
}

pub fn part_two() -> String {
    String::new()
}
