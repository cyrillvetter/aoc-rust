use super::common::solve_parts;

pub fn solve_day(day: u8) {
    match day {
        4 => solve_parts(day, day_04::part_one, day_04::part_two),
        _ => return,
    };
}

mod day_04;
