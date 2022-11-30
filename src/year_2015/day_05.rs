use crate::common::read_lines;
use itertools::Itertools;
use std::str;

pub fn part_one() -> String {
    let lines = read_lines(2015, 5);
    let vowels: [char; 5] = [ 'a', 'e', 'i', 'o', 'u' ];
    let disallowed: [&str; 4] = [ "ab", "cd", "pq", "xy" ];

    let mut count = 0;
    for l in lines {
        if !disallowed.iter().any(|f| l.contains(f)) &&
           l.split(vowels).count() > 3 &&
           l.chars().tuple_windows::<(_, _)>().any(|t| t.0 == t.1) {
            count += 1;
        }
    };

    return count.to_string();
}

pub fn part_two() -> String {
    let lines = read_lines(2015, 5);

    let mut count = 0;
    for l in lines {
        if l.chars().tuple_windows::<(_, _, _)>().any(|pair| pair.0 == pair.2) &&
           // TODO: Find better way to check for repeating chars (replace rfind)
           l.chars().tuple_windows::<(_, _)>().enumerate().any(|(i, pair)| l.rfind(str::from_utf8(&[pair.0 as u8, pair.1 as u8]).unwrap()).map(|j| j > i + 1).unwrap_or(false)) {
            count += 1;
        }
    };

    return count.to_string();
}
