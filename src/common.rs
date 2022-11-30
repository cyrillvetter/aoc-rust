use std::fs;
use std::time::Instant;

pub fn read_lines(year: i32, day: i32) -> Vec<String> {
    // TODO: Implement better way to read by lines.
    read_string(year, day).lines().map(|l| String::from(l)).collect()
}

pub fn read_string(year: i32, day: i32) -> String {
    let path = format!("./src/year_{}/inputs/{}.txt", year, day);
    fs::read_to_string(&path).expect("File not found")
}

pub fn solve_parts(year: u32, day: u8, func_one: impl FnOnce() -> String, func_two: impl FnOnce() -> String) {
    println!("# Executing {} Day {} ...", year, day);
    solve(func_one, 1);
    solve(func_two, 2);
    println!();
}

fn solve(func: impl FnOnce() -> String, part: u8) {
    let now = Instant::now();
    let result = func();
    let elapsed_ms = (now.elapsed().as_micros() as f64) / 1_000.0;

    println!("Part {}: {} ({}ms)", part, result, elapsed_ms);
}
