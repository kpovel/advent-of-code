use std::{collections::HashMap, fs};

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);

    let second_part = solve_second_part();
    println!("Second part: {}", second_part);
}

fn solve_second_part() -> u32 {
    let file_content = fs::read_to_string("input").unwrap();
    let mut current_directory = String::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for line in file_content.trim().split("\n").into_iter() {
        let trim_line = line.trim();
        match trim_line.starts_with("$ cd") {
            true => change_directory(&mut current_directory, &trim_line),
            false => process_list(&current_directory, &mut dirs, &trim_line),
        }
    }

    deleted_dir_size(&dirs)
}

fn deleted_dir_size(dirs: &HashMap<String, u32>) -> u32 {
    let mut dirs_size: Vec<&u32> = dirs.values().collect();
    dirs_size.sort();

    for x in &dirs_size {
        if *dirs_size.last().unwrap() - *x < 40_000_000 {
            return **x;
        }
    }

    panic!("Are use right data?");
}

fn solve_first_part() -> u32 {
    let file_content = fs::read_to_string("input").unwrap();
    let mut current_directory = String::new();
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for line in file_content.trim().split("\n").into_iter() {
        let trim_line = line.trim();
        match trim_line.starts_with("$ cd") {
            true => change_directory(&mut current_directory, &trim_line),
            false => process_list(&current_directory, &mut dirs, &trim_line),
        }
    }

    total_size(&dirs)
}

fn total_size(dirs: &HashMap<String, u32>) -> u32 {
    let mut sum = 0;
    for i in dirs.values().into_iter() {
        if *i < 100_000 {
            sum += i;
        }
    }

    sum
}

fn process_list(current_dir: &str, dirs: &mut HashMap<String, u32>, dir_content: &str) {
    let split_content: Vec<&str> = dir_content.split_whitespace().collect();
    let is_file = !split_content[0].starts_with("$") && !split_content[0].starts_with("dir");

    if is_file {
        let file_size = split_content[0].parse::<u32>().unwrap();
        let mut parent_path = current_dir.to_string();

        loop {
            *dirs.entry(parent_path.clone()).or_insert(0) += file_size;
            if parent_path == "/" {
                break;
            }
            parent_path = match parent_path.rfind('/') {
                Some(index) if index > 0 => parent_path[..index].to_string(),
                _ => String::from("/"),
            };
        }
    }
}

fn change_directory(current_dir: &mut String, move_command: &str) {
    match move_command {
        "$ cd /" => *current_dir = String::from("/"),
        "$ cd .." => {
            let parent_path = match current_dir.rfind('/') {
                Some(index) => &current_dir[..index],
                None => &current_dir,
            };

            match parent_path == "" {
                true => *current_dir = String::from("/"),
                false => *current_dir = parent_path.to_string(),
            };
        }
        _ => {
            let move_in = &move_command[5..];

            match current_dir == "/" {
                true => current_dir.push_str(move_in),
                false => current_dir.push_str(&format!("/{}", move_in)),
            };
        }
    }
}
