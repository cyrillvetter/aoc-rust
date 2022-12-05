use super::*;
use crate::solution::Solution;
use crate::common::read_string;

#[test]
fn day_01() {
    let input = read_string(2016, 1);
    assert_eq!(day_01::part_one(&input), Solution::I32(273));
    assert_eq!(day_01::part_two(&input), Solution::I32(115));
}

#[test]
fn day_02() {
    let input = read_string(2016, 2);
    assert_eq!(day_02::part_one(&input), Solution::Str(String::from("99332")));
    assert_eq!(day_02::part_two(&input), Solution::Str(String::from("DD483")));
}
