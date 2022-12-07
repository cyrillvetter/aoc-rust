use crate::solution::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Solution {
    const MAX_SIZE: u32 = 100000;
    let dirs = build_file_system(input);

    let file_sum = dirs
        .iter()
        .map(|(k, _)| calculate_total_file_size(&dirs, k))
        .filter(|sum| *sum < MAX_SIZE)
        .sum::<u32>();

    Solution::U32(file_sum)
}

pub fn part_two(input: &str) -> Solution {
    const TOTAL_DISK_SPACE: u32 = 70000000;
    const NEEDED_FREE_SPACE: u32 = 30000000;
    let dirs = build_file_system(input);

    let free_space = TOTAL_DISK_SPACE - calculate_total_file_size(&dirs, "/");

    let smallest_dir = dirs
        .iter()
        .map(|(k, _)| calculate_total_file_size(&dirs, k))
        .filter(|sum| free_space + *sum > NEEDED_FREE_SPACE)
        .min().unwrap();

    Solution::U32(smallest_dir)
}

struct Directory {
    dirs: Vec<String>,
    file_sizes: Vec<u32>
}

fn build_file_system(input: &str) -> HashMap<String, Directory> {
    let mut dirs: HashMap<String, Directory> = HashMap::new();
    let mut curr: String = String::from("/");
    dirs.insert(curr.clone(), Directory { dirs: Vec::new(), file_sizes: Vec::new() });

    for l in input.lines().skip(1) {
        if l.starts_with("$") {
            let parts = l.split_whitespace().skip(1).collect_vec();
            match parts[0] {
                "cd" => {
                    match parts[1] {
                        ".." => {
                            let last_separator = curr.rfind("/").unwrap();
                            curr.replace_range(last_separator..curr.len(), "");
                        },
                        _ => move_in_dir(&mut curr, parts[1]),
                    }
                },
                _ => (),
            }
        } else {
            let parts = l.split_once(" ").unwrap();
            match parts.0 {
                "dir" => {
                    let mut current_dir = curr.clone();
                    move_in_dir(&mut current_dir, parts.1);
                    dirs.get_mut(&curr).unwrap().dirs.push(current_dir.clone());
                    dirs.insert(current_dir, Directory { dirs: Vec::new(), file_sizes: Vec::new() });
                },
                _ => {
                    let size = parts.0.parse::<u32>().unwrap();
                    dirs.get_mut(&curr).unwrap().file_sizes.push(size);
                }
            }
        }
    }

    dirs
}

fn calculate_total_file_size(dirs: &HashMap<String, Directory>, dir_name: &str) -> u32 {
    let dir = dirs.get(dir_name).unwrap();
    let mut total = dir.file_sizes.iter().sum::<u32>();
    for childs in &dir.dirs {
        total += calculate_total_file_size(dirs, &childs);
    }

    total
}

fn move_in_dir(path: &mut String, dir: &str) {
    if !path.ends_with("/") {
        path.push_str("/");
    }

    path.push_str(dir);
}
