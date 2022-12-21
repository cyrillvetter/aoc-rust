use std::time::Instant;
use crate::common::solve_parts;

const YEAR: u32 = 2022;

pub fn solve_all() {
    for i in 1..25 {
        solve_day(i)
    }
}

pub fn solve_day(day: u8) {
    match day {
        1 => solve_parts(YEAR, day, day_01::part_one, day_01::part_two),
        2 => solve_parts(YEAR, day, day_02::part_one, day_02::part_two),
        3 => solve_parts(YEAR, day, day_03::part_one, day_03::part_two),
        4 => solve_parts(YEAR, day, day_04::part_one, day_04::part_two),
        5 => solve_parts(YEAR, day, day_05::part_one, day_05::part_two),
        6 => solve_parts(YEAR, day, day_06::part_one, day_06::part_two),
        7 => solve_parts(YEAR, day, day_07::part_one, day_07::part_two),
        8 => solve_parts(YEAR, day, day_08::part_one, day_08::part_two),
        9 => solve_parts(YEAR, day, day_09::part_one, day_09::part_two),
        10 => solve_parts(YEAR, day, day_10::part_one, day_10::part_two),
        11 => solve_parts(YEAR, day, day_11::part_one, day_11::part_two),
        12 => solve_parts(YEAR, day, day_12::part_one, day_12::part_two),
        13 => solve_parts(YEAR, day, day_13::part_one, day_13::part_two),
        14 => solve_parts(YEAR, day, day_14::part_one, day_14::part_two),
        15 => solve_parts(YEAR, day, day_15::part_one, day_15::part_two),
        18 => solve_parts(YEAR, day, day_18::part_one, day_18::part_two),
        20 => solve_parts(YEAR, day, day_20::part_one, day_20::part_two),
        21 => solve_parts(YEAR, day, day_21::part_one, day_21::part_two),
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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_18;
mod day_20;
mod day_21;
