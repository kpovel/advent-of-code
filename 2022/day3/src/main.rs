use std::fs;

fn main() {
    let file_content = fs::read_to_string("./input").expect("file");
    let repeat_compartment = find_repeat_compartment(&file_content);
    let prioritized_compartmens = prioritize_compartment(&repeat_compartment);

    println!("Part 1: {}", prioritized_compartmens);

    let repeat_compartment_group = group_repeat_compartment(&file_content);
    let group_prior = prioritize_compartment(&repeat_compartment_group);
    println!("Part 2: {}", group_prior);
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

fn group_repeat_compartment(input: &str) -> String {
    let splited_groups = split_groups(&input);
    let mut repeat_items = String::from("");

    for group in splited_groups.into_iter() {
        let elves: Vec<&str> = group.split("\n").into_iter().collect();

        for c in elves[0].chars().into_iter() {
            match (elves[1].contains(c), elves[2].contains(c)) {
                (true, true) => {
                    repeat_items.push_str(&c.to_string());
                    break;
                }
                _ => (),
            }
        }
    }

    repeat_items
}

fn split_groups(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut split_groups: Vec<String> = vec![];

    for chank in lines.chunks(3) {
        split_groups.push(chank.join(
            "
",
        ));
    }

    split_groups
}
