use crate::solution::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn part_one(input: &str) -> Solution {
    const MAX_SIZE: u32 = 100_000;
    let dirs = build_file_system(input);

    let file_sum = dirs
        .iter()
        .map(|(k, _)| calculate_dir_size(&dirs, k))
        .filter(|dir_size| *dir_size < MAX_SIZE)
        .sum();

    Solution::U32(file_sum)
}

pub fn part_two(input: &str) -> Solution {
    const TOTAL_DISK_SPACE: u32 = 70_000_000;
    const NEEDED_FREE_SPACE: u32 = 30_000_000;
    let dirs = build_file_system(input);

    let free_space = TOTAL_DISK_SPACE - calculate_dir_size(&dirs, &PathBuf::from("/"));

    let smallest_dir = dirs
        .iter()
        .map(|(k, _)| calculate_dir_size(&dirs, k))
        .filter(|dir_size| free_space + *dir_size > NEEDED_FREE_SPACE)
        .min().unwrap();

    Solution::U32(smallest_dir)
}

struct Directory {
    dirs: Vec<PathBuf>,
    files_size: u32,
}

fn build_file_system(input: &str) -> HashMap<PathBuf, Directory> {
    let mut dirs: HashMap<PathBuf, Directory> = HashMap::new();
    let mut curr: PathBuf = PathBuf::from("/");
    dirs.insert(curr.clone(), Directory { dirs: Vec::new(), files_size: 0 });

    for l in input.lines().skip(1) {
        if l.starts_with('$') {
            let parts = l.split_whitespace().skip(1).collect_vec();
            if parts[0] == "cd" {
                match parts[1] {
                    ".." => { curr.pop(); },
                    _ => curr.push(parts[1]),
                }
            }
        } else {
            let parts = l.split_once(' ').unwrap();
            match parts.0 {
                "dir" => {
                    let mut current_dir = curr.clone();
                    current_dir.push(parts.1);
                    dirs.get_mut(&curr).unwrap().dirs.push(current_dir.clone());
                    dirs.insert(current_dir, Directory { dirs: Vec::new(), files_size: 0 });
                },
                _ => dirs.get_mut(&curr).unwrap().files_size += parts.0.parse::<u32>().unwrap(),
            }
        }
    }

    dirs
}

fn calculate_dir_size(dirs: &HashMap<PathBuf, Directory>, dir_name: &PathBuf) -> u32 {
    let dir = dirs.get(dir_name).unwrap();
    let mut total = dir.files_size;
    for childs in &dir.dirs {
        total += calculate_dir_size(dirs, childs);
    }

    total
}
