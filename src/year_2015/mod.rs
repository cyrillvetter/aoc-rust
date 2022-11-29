use crate::common::solve_parts;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(day, day_01::part_one, day_01::part_two),
        2 => solve_parts(day, day_02::part_one, day_02::part_two),
        3 => solve_parts(day, day_03::part_one, day_03::part_two),
        4 => solve_parts(day, day_04::part_one, day_04::part_two),
        5 => solve_parts(day, day_05::part_one, day_05::part_two),
        6 => solve_parts(day, day_06::part_one, day_06::part_two),
        _ => return,
    };
}

#[cfg(test)]
mod tests;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
