use crate::solution::Solution;
use std::fs;
use std::time::Instant;

pub fn read_string(year: u32, day: u8) -> String {
    let path = format!("./src/year_{}/inputs/{}.txt", year, day);
    fs::read_to_string(&path).expect("File not found").replace("\r\n", "\n")
}

pub fn solve_parts(year: u32, day: u8, func_one: impl FnOnce(&str) -> Solution, func_two: impl FnOnce(&str) -> Solution) {
    println!("# Executing {} Day {} ...", year, day);

    let input = read_string(year, day);
    solve(func_one, &input, 1);
    solve(func_two, &input, 2);

    println!();
}

fn solve(func: impl FnOnce(&str) -> Solution, input: &str, part: u8) {
    let now = Instant::now();
    let result = func(input);
    let elapsed_ms = (now.elapsed().as_micros() as f64) / 1_000.0;

    println!("Part {}: {} ({}ms)", part, result, elapsed_ms);
}
