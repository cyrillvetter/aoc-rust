use crate::common::solve_parts;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(2016, day, day_01::part_one, day_01::part_two),
        2 => solve_parts(2016, day, day_02::part_one, day_02::part_two),
        _ => return,
    };
}

mod day_01;
mod day_02;
