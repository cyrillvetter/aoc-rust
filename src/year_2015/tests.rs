use super::*;
use crate::solution::Solution;
use crate::common::read_string;

#[test]
fn day_01() {
    let input = read_string(2015, 1);
    assert_eq!(day_01::part_one(&input), Solution::I32(74));
    assert_eq!(day_01::part_two(&input), Solution::USize(1795));
}

#[test]
fn day_02() {
    let input = read_string(2015, 2);
    assert_eq!(day_02::part_one(&input), Solution::I32(1606483));
    assert_eq!(day_02::part_two(&input), Solution::I32(3842356));
}

#[test]
fn day_03() {
    let input = read_string(2015, 3);
    assert_eq!(day_03::part_one(&input), Solution::USize(2592));
    assert_eq!(day_03::part_two(&input), Solution::USize(2360));
}

#[test]
fn day_04() {
    let input = read_string(2015, 4);
    assert_eq!(day_04::part_one(&input), Solution::U32(254575));
    assert_eq!(day_04::part_two(&input), Solution::U32(1038736));
}

#[test]
fn day_05() {
    let input = read_string(2015, 5);
    assert_eq!(day_05::part_one(&input), Solution::U32(258));
    assert_eq!(day_05::part_two(&input), Solution::U32(53));
}

#[test]
fn day_06() {
    let input = read_string(2015, 6);
    assert_eq!(day_06::part_one(&input), Solution::I32(569999));
    assert_eq!(day_06::part_two(&input), Solution::I32(17836115));
}
