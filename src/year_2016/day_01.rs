use crate::solution::Solution;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Solution {
    let mut loc: (i32, i32) = (0, 0);
    let mut degress = 0;

    for d in input.split(", ").into_iter() {
        let (direction, steps) = d.split_at(1);
        let steps = steps.parse::<i32>().unwrap();

        degress = match direction {
            "R" => calculate_degrees(degress, 90),
            "L" => calculate_degrees(degress, -90),
            _ => unreachable!(),
        };

        loc = match degress {
            0 => (loc.0 + steps, loc.1),
            90 => (loc.0, loc.1 + steps),
            180 => (loc.0 - steps, loc.1),
            270 => (loc.0, loc.1 - steps),
            _ => unreachable!(),
        }
    }

    Solution::I32(loc.0.abs() + loc.1.abs())
}

pub fn part_two(input: &str) -> Solution {
    let mut loc: (i32, i32) = (0, 0);
    let mut degress = 0;
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    visits.insert(loc);

    for d in input.split(", ").into_iter() {
        let (direction, steps) = d.split_at(1);
        let steps = steps.parse::<i32>().unwrap();

        degress = match direction {
            "R" => calculate_degrees(degress, 90),
            "L" => calculate_degrees(degress, -90),
            _ => unreachable!(),
        };

        let mut inter_loc = loc;

        for l in 1..=steps {
            inter_loc = match degress {
                0 => (loc.0 + l, loc.1),
                90 => (loc.0, loc.1 + l),
                180 => (loc.0 - l, loc.1),
                270 => (loc.0, loc.1 - l),
                _ => unreachable!(),
            };

            if visits.contains(&inter_loc) {
                return Solution::I32(inter_loc.0.abs() + inter_loc.1.abs());
            }

            visits.insert(inter_loc);
        }

        loc = inter_loc;
    }

    unreachable!()
}

fn calculate_degrees(curr: i32, next: i32) -> i32 {
    let n = curr + next;
    if n == -90 {
        return 270;
    } else if n == 360 {
        return 0;
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2016, 1);
        assert_eq!(part_one(&input), Solution::I32(273));
        assert_eq!(part_two(&input), Solution::I32(115));
    }
}