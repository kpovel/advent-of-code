use std::fs;

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);

    let second_part = solve_second_part();
    println!("Second part: {}", second_part);
}

fn solve_second_part() -> u32 {
    find_start_marker(14)
}

fn solve_first_part() -> u32 {
    find_start_marker(4)
}

fn find_start_marker(distinct_characters: u32) -> u32 {
    let file_content = fs::read_to_string("input").unwrap();

    for x in 0..file_content.len() {
        let buffer_part = &file_content[x..x + distinct_characters as usize];
        if is_marker(&buffer_part) {
            return x as u32 + distinct_characters;
        }
    }

    panic!("Maybe your puzzle input is wrong");
}

fn is_marker(characters: &str) -> bool {
    for c in characters.chars() {
        let repeated_char: Vec<char> = characters.chars().filter(|x| *x == c).collect();
        if repeated_char.len() > 1 {
            return false;
        }
    }

    true
}
