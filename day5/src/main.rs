use std::fs;

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);
}

fn solve_first_part() -> String {
    let file_content = fs::read_to_string("input").unwrap();
    let split_content: Vec<&str> = file_content.split("\n\n").collect();
    let (raw_crates, procedures) = (split_content[0], split_content[1]);
    let mut crate_stack = build_crates(raw_crates);

    move_crates(&mut crate_stack, procedures);

    top_crates(&crate_stack)
}

fn top_crates(crate_stack: &Vec<String>) -> String {
    crate_stack
        .iter()
        .filter_map(|s| s.chars().last())
        .collect()
}

fn move_crates(crate_stack: &mut Vec<String>, procedures: &str) {
    for p in procedures.trim().split("\n") {
        let split_procedure: Vec<&str> = p.split(" ").collect();
        let (move_crates, move_from, move_to) = (
            split_procedure[1].parse::<usize>().unwrap(),
            split_procedure[3].parse::<usize>().unwrap(),
            split_procedure[5].parse::<usize>().unwrap(),
        );

        for _ in 0..move_crates {
            let moved_crate = crate_stack[move_from - 1].pop().unwrap();
            crate_stack[move_to - 1].push(moved_crate);
        }
    }
}

fn build_crates(raw_crates: &str) -> Vec<String> {
    let mut crate_elements: Vec<String> = vec![];

    let crate_stacks = raw_crates
        .split("\n")
        .last()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();

    for _ in 0..crate_stacks {
        crate_elements.push(String::from(""));
    }

    for l in raw_crates.split("\n") {
        if l.trim().starts_with("1") {
            break;
        }

        for x in 0..crate_stacks {
            let mut l_char = l.chars();
            match l_char.nth(4 * x + 1) {
                Some(c) => {
                    crate_elements[x] = format!("{}{}", c.to_string().trim(), crate_elements[x])
                }
                None => (),
            };
        }
    }

    crate_elements
}
