use crate::solution::Solution;
use md5;

pub fn part_one(input: &str) -> Solution {
    let zero_count = 5;
    Solution::U32(compute_hash(input, zero_count))
}

pub fn part_two(input: &str) -> Solution {
    let zero_count = 6;
    Solution::U32(compute_hash(input, zero_count))
}

fn compute_hash(input: &str, zero_count: usize) -> u32 {
    let half_len = zero_count / 2;
    let is_even = zero_count % 2 == 0;
    let compare_vec = vec![0; half_len];

    let mut i = 0;

    loop {
        let hash = md5::compute(input.to_owned() + &i.to_string());
        let hash_parts = hash.0;

        if hash_parts[..half_len] == compare_vec {
            if !is_even {
                if hash_parts[half_len] < 16 {
                    return i;
                }
            } else {
                return i;
            }
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use super::*;
    use crate::common::read_string;

    #[test]
    fn check() {
        let input = read_string(2015, 4);
        assert_eq!(part_one(&input), Solution::U32(254575));
        assert_eq!(part_two(&input), Solution::U32(1038736));
    }
}
