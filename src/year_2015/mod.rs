use crate::common::solve_parts;

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(day, day_01::part_one, day_01::part_two),
        2 => solve_parts(day, day_02::part_one, day_02::part_two),
        4 => solve_parts(day, day_04::part_one, day_04::part_two),
        _ => return,
    };
}

mod day_01;
mod day_02;
mod day_04;
