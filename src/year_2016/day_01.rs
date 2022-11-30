use crate::common::read_string;
use std::collections::HashSet;

pub fn part_one() -> String {
    let mut loc: (i32, i32) = (0, 0);
    let mut degress = 0;

    for d in read_string(2016, 1).split(", ").into_iter() {
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

    (loc.0.abs() + loc.1.abs()).to_string()
}

pub fn part_two() -> String {
    let mut loc: (i32, i32) = (0, 0);
    let mut degress = 0;
    let mut visits: HashSet<(i32, i32)> = HashSet::new();
    visits.insert(loc);

    for d in read_string(2016, 1).split(", ").into_iter() {
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
                return (inter_loc.0.abs() + inter_loc.1.abs()).to_string();
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
