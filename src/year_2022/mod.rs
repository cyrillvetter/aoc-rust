use crate::common::solve_parts;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(2022, day, day_01::part_one, day_01::part_two),
        2 => solve_parts(2022, day, day_02::part_one, day_02::part_two),
        3 => solve_parts(2022, day, day_03::part_one, day_03::part_two),
        4 => solve_parts(2022, day, day_04::part_one, day_04::part_two),
        5 => solve_parts(2022, day, day_05::part_one, day_05::part_two),
        6 => solve_parts(2022, day, day_06::part_one, day_06::part_two),
        7 => solve_parts(2022, day, day_07::part_one, day_07::part_two),
        8 => solve_parts(2022, day, day_08::part_one, day_08::part_two),
        9 => solve_parts(2022, day, day_09::part_one, day_09::part_two),
        10 => solve_parts(2022, day, day_10::part_one, day_10::part_two),
        _ => (),
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
mod day_07;
mod day_08;
mod day_09;
mod day_10;
