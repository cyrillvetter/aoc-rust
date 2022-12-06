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
