use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input").expect("file");
    let repeat_compartment = find_repeat_compartment(&file_content);
    let prioritized_compartmens = prioritize_compartment(&repeat_compartment);

    println!("{}", prioritized_compartmens);
}

fn prioritize_compartment(compartments: &str) -> u32 {
    compartments
        .chars()
        .into_iter()
        .map(|s| match s {
            'a'..='z' => s as u32 - 'a' as u32 + 1,
            'A'..='Z' => s as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid item: {}", s),
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn find_repeat_compartment(input: &str) -> String {
    let mut repeat_items = String::from("");

    let split_line: Vec<(&str, &str)> = input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| line.split_at(line.len() / 2))
        .collect();

    for (first, second) in split_line {
        for c in first.chars().into_iter() {
            match second.contains(c) {
                true => {
                    repeat_items.push_str(&c.to_string());
                    break;
                }
                false => (),
            };
        }
    }

    repeat_items
}
