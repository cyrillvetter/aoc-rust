use std::time::Instant;

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
