use crate::solution::Solution;

pub fn part_one(input: &str) -> Solution {
    let mut score = 0;

    for line in input.lines() {
        let split = line.split_at(1);

        let a_score = get_choice_score(split.0);
        let mut b_score = get_choice_score(split.1.trim());
        if a_score == b_score {
            b_score += 3;
        } else if a_score == 1 && b_score == 2 || a_score == 2 && b_score == 3 || a_score == 3 && b_score == 1 {
            b_score += 6;
        }

        score += b_score;
    }

    Solution::U32(score)
}

pub fn part_two(input: &str) -> Solution {
    let mut score = 0;

    for line in input.lines() {
        let split = line.split_at(1);

        let a_score = get_choice_score(split.0);
        score += match split.1.trim() {
            "X" => if a_score > 1 { a_score - 1 } else { 3 },
            "Y" => a_score + 3,
            "Z" => (if a_score < 3 { a_score + 1 } else { 1 }) + 6,
            _ => unreachable!()
        }
    }

    Solution::U32(score)
}

fn get_choice_score(choice: &str) -> u32 {
    match choice {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!()
    }
}
