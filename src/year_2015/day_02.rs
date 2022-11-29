use crate::common::read_lines;

pub fn part_one() -> String {
    read_lines(2015, 2)
        .iter()
        .map(|p| p.split('x').map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .map(|mut s| {
            let result = 2 * s[0] * s[1] + 2 * s[1] * s[2] + 2 * s[2] * s[0];
            s.sort();
            result + s[0] * s[1]
        })
        .sum::<i32>()
        .to_string()
}

pub fn part_two() -> String {
    read_lines(2015, 2)
        .iter()
        .map(|p| p.split('x').map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .map(|mut s| {
            s.sort();
            2 * s[0] + 2 * s[1] + s[0] * s[1] * s[2]
        })
        .sum::<i32>()
        .to_string()
}
