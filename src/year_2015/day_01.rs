use crate::common::read_string;

pub fn part_one() -> String {
    read_string(2015, 1)
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>()
        .to_string()
}

pub fn part_two() -> String {
    let mut floor: i32 = 0;
    for (i, n) in read_string(2015, 1).chars().map(|c| if c == '(' { 1i32 } else { -1i32 }).enumerate() {
        floor += n;

        if floor < 0 {
            return (i + 1).to_string()
        }
    };

    unreachable!();
}
