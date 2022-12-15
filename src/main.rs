extern crate core;

fn main() {
    println!("🎄 Advent of Code 🎄");

    //year_2015::solve_all();
    //year_2016::solve_all();
    //year_2022::solve_all();
    year_2022::solve_day(15);
}

mod common;
mod solution;

mod year_2015;
mod year_2016;
mod year_2022;
