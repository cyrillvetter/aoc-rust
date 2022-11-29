use crate::common::read_lines;
use std::str;

pub fn part_one() -> String {
    let lines = read_lines(2015, 5);
    let vowels: [char; 5] = [ 'a', 'e', 'i', 'o', 'u' ];
    let disallowed: [&str; 4] = [ "ab", "cd", "pq", "xy" ];

    let mut count = 0;
    for l in lines {
        if disallowed.iter().any(|f| l.contains(f)) {
            continue;
        }

        if l.split(vowels).count() <= 3 {
            continue;
        }

        if !l.chars().collect::<Vec<char>>().windows(2).any(|pair| pair[0] == pair[1]) {
            continue;
        }

        count += 1;
    };

    return count.to_string();
}

pub fn part_two() -> String {
    let lines = read_lines(2015, 5);

    let mut count = 0;
    for l in lines {
        let chars = l.chars().collect::<Vec<char>>();

        if !chars.windows(3).any(|pair| pair[0] == pair[2]) {
            continue;
        }

        if !chars.windows(2).enumerate().any(|(i, pair)| l.rfind(&pair.iter().collect::<String>()).map(|j| j > i + 1).unwrap_or(false)) {
            continue;
        }

        count += 1;
    };

    return count.to_string();
}
