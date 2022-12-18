use super::*;
use crate::solution::Solution;
use crate::common::read_string;

#[test]
fn day_01() {
    let input = read_string(2022, 1);
    assert_eq!(day_01::part_one(&input), Solution::I32(70613));
    assert_eq!(day_01::part_two(&input), Solution::I32(205805));
}

#[test]
fn day_02() {
    let input = read_string(2022, 2);
    assert_eq!(day_02::part_one(&input), Solution::U32(11767));
    assert_eq!(day_02::part_two(&input), Solution::U32(13886));
}

#[test]
fn day_03() {
    let input = read_string(2022, 3);
    assert_eq!(day_03::part_one(&input), Solution::U32(7581));
    assert_eq!(day_03::part_two(&input), Solution::U32(2525));
}

#[test]
fn day_04() {
    let input = read_string(2022, 4);
    assert_eq!(day_04::part_one(&input), Solution::USize(530));
    assert_eq!(day_04::part_two(&input), Solution::USize(903));
}

#[test]
fn day_05() {
    let input = read_string(2022, 5);
    assert_eq!(day_05::part_one(&input), Solution::Str(String::from("ZSQVCCJLL")));
    assert_eq!(day_05::part_two(&input), Solution::Str(String::from("QZFJRWHGS")));
}

#[test]
fn day_06() {
    let input = read_string(2022, 6);
    assert_eq!(day_06::part_one(&input), Solution::USize(1896));
    assert_eq!(day_06::part_two(&input), Solution::USize(3452));
}

#[test]
fn day_07() {
    let input = read_string(2022, 7);
    assert_eq!(day_07::part_one(&input), Solution::U32(1432936));
    assert_eq!(day_07::part_two(&input), Solution::U32(272298));
}

#[test]
fn day_08() {
    let input = read_string(2022, 8);
    assert_eq!(day_08::part_one(&input), Solution::USize(1805));
    assert_eq!(day_08::part_two(&input), Solution::USize(444528));
}

#[test]
fn day_09() {
    let input = read_string(2022, 9);
    assert_eq!(day_09::part_one(&input), Solution::USize(6271));
    assert_eq!(day_09::part_two(&input), Solution::USize(2458));
}

#[test]
fn day_10() {
    let input = read_string(2022, 10);
    assert_eq!(day_10::part_one(&input), Solution::I32(14320));
    let expected_part2 = "\n\
    ###...##..###..###..#..#..##..###....##.\n\
    #..#.#..#.#..#.#..#.#.#..#..#.#..#....#.\n\
    #..#.#....#..#.###..##...#..#.#..#....#.\n\
    ###..#....###..#..#.#.#..####.###.....#.\n\
    #....#..#.#....#..#.#.#..#..#.#....#..#.\n\
    #.....##..#....###..#..#.#..#.#.....##..";
    assert_eq!(day_10::part_two(&input), Solution::Str(String::from(expected_part2)));
}

#[test]
fn day_11() {
    let input = read_string(2022, 11);
    assert_eq!(day_11::part_one(&input), Solution::U64(113220));
    assert_eq!(day_11::part_two(&input), Solution::U64(30599555965));
}

#[test]
fn day_12() {
    let input = read_string(2022, 12);
    assert_eq!(day_12::part_one(&input), Solution::USize(497));
    assert_eq!(day_12::part_two(&input), Solution::USize(492));
}

#[test]
fn day_13() {
    let input = read_string(2022, 13);
    assert_eq!(day_13::part_one(&input), Solution::USize(5682));
    assert_eq!(day_13::part_two(&input), Solution::USize(20304));
}

#[test]
fn day_14() {
    let input = read_string(2022, 14);
    assert_eq!(day_14::part_one(&input), Solution::U32(825));
    assert_eq!(day_14::part_two(&input), Solution::U32(26729));
}

#[test]
fn day_15() {
    let input = read_string(2022, 15);
    assert_eq!(day_15::part_one(&input), Solution::U32(5299855));
    assert_eq!(day_15::part_two(&input), Solution::U64(13615843289729));
}

#[test]
fn day_18() {
    let input = read_string(2022, 18);
    assert_eq!(day_18::part_one(&input), Solution::USize(3662));
    assert_eq!(day_18::part_two(&input), Solution::Empty);
}
