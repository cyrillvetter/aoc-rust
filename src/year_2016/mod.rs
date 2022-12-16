use crate::common::solve_parts;

const YEAR: u32 = 2016;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(YEAR, day, day_01::part_one, day_01::part_two),
        2 => solve_parts(YEAR, day, day_02::part_one, day_02::part_two),
        _ => (),
    };
}

#[cfg(test)]
mod tests;

mod day_01;
mod day_02;
