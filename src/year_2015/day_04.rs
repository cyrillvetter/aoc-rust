use md5;

pub fn part_one() -> String {
    let input = "bgvyzdsv";
    let expected_start = "00000";
    compute_hash(input, expected_start).to_string()
}

pub fn part_two() -> String {
    let input = "bgvyzdsv";
    let expected_start = "000000";
    compute_hash(input, expected_start).to_string()
}

fn compute_hash(input: &str, expected_begin: &str) -> u32 {
    let mut i = 0;

    loop {
        let hash = md5::compute(input.to_owned() + &i.to_string());
        let hash_str = format!("{:?}", hash);
        if hash_str.starts_with(expected_begin) {
            return i;
        }

        i += 1;
    }
}
