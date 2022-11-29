use crate::common::read_lines;

pub fn part_one() -> String {
    read_lines(2015, 2)
        .iter()
        .map(|p| p.split('x').map(|s| s.parse().unwrap()).collect::<Vec<i32>>())
        .map(|s| (s[0], s[1], s[2]))
        .map(|f| (f.0 * f.1, f.1 * f.2, f.2 * f.0))
        .map(|r| 2 * r.0 + 2 * r.1 + 2 * r.2 + [r.0, r.1, r.2].iter().min().unwrap())
        .sum::<i32>()
        .to_string()
}

pub fn part_two() -> String {
    String::new()
}
