use crate::common::solve_parts;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(2022, day, day_01::part_one, day_01::part_two),
        _ => return,
    };
}

#[cfg(test)]
mod tests;

mod day_01;
