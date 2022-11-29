use md5;

pub fn part_one() -> String {
    let input = "bgvyzdsv";
    let zero_count = 5;

    compute_hash(input, zero_count).to_string()
}

pub fn part_two() -> String {
    let input = "bgvyzdsv";
    let zero_count = 6;

    compute_hash(input, zero_count).to_string()
}

fn compute_hash(input: &str, zero_count: usize) -> u32 {
    let mid = zero_count / 2;
    let is_even = zero_count % 2 == 0;
    let compare_vec = vec![0; mid];

    let mut i = 0;

    loop {
        let hash = md5::compute(input.to_owned() + &i.to_string());
        let hash_parts = hash.0;

        if is_even {
            if hash_parts[..mid] == compare_vec {
                return i;
            }
        } else {
            if hash_parts[..mid] == compare_vec && hash_parts[mid] < 16 {
                return i;
            }
        }

        i += 1;
    }
}
