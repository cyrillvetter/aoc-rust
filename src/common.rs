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

pub fn solve_parts(day: u8, func_one: impl FnOnce() -> String, func_two: impl FnOnce() -> String) {
    println!("Executing day {} ...", day);
    solve(func_one);
    solve(func_two);
}

fn solve(func: impl FnOnce() -> String) {
    let now = Instant::now();
    let result = func();
    let elapsed_ms = now.elapsed().as_millis();

    println!("Part 1: {} ({}ms)", result, elapsed_ms);
}
