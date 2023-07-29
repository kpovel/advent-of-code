use std::fs;

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);
}

fn solve_first_part() -> u32 {
    let file_content = fs::read_to_string("input").unwrap();

    for x in 0..file_content.len() {
        let buffer_part = &file_content[x..x + 4];
        if is_marker(&buffer_part) {
            return x as u32 + 4;
        }
    }

    69
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
