use crate::common::read_lines;
use std::str;

pub fn part_one() -> String {
    let lines = read_lines(2015, 5);
    let vowels: [char; 5] = [ 'a', 'e', 'i', 'o', 'u' ];
    let disallowed: [&str; 4] = [ "ab", "cd", "pq", "xy" ];

    let mut count = 0;
    for l in lines {
        if !disallowed.iter().any(|f| l.contains(f)) &&
           l.split(vowels).count() > 3 &&
           l.as_bytes().windows(2).any(|t| t[0] == t[1]) {
            count += 1;
        }
    };

    return count.to_string();
}

pub fn part_two() -> String {
    let lines = read_lines(2015, 5);

    let mut count = 0;
    for l in lines {
        if l.as_bytes().windows(3).any(|pair| pair[0] == pair[2]) &&
           // TODO: Find better way to check for repeating chars (replace rfind and as_bytes)
           l.as_bytes().windows(2).enumerate().any(|(i, pair)| l.rfind(str::from_utf8(pair).unwrap()).map(|j| j > i + 1).unwrap_or(false)) {
            count += 1;
        }
    };

    return count.to_string();
}
