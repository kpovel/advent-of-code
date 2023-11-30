use std::fs;

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);

    let second_part = solve_second_task();
    println!("Second part: {}", second_part);
}

fn solve_first_part() -> u32 {
    let mut fully_contains = 0;

    let file_content = fs::read_to_string("input").unwrap();

    for l in file_content.trim().split("\n").into_iter() {
        let (first, second, third, fourth) = define_numbers(&l);

        if first <= third && second >= fourth || first >= third && second <= fourth {
            fully_contains += 1;
        }
    }

    fully_contains
}

fn solve_second_task() -> u32 {
    let file_content = fs::read_to_string("input").unwrap();
    let mut overlap = 0;

    for l in file_content.trim().split("\n").into_iter() {
        let (first, second, third, fourth) = define_numbers(&l);

        if second >= third && first <= fourth {
            overlap += 1;
        }
    }

    overlap
}

fn define_numbers(line: &str) -> (u32, u32, u32, u32) {
    let nums: Vec<u32> = line
        .split(&['-', ',', '-'][..])
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    (nums[0], nums[1], nums[2], nums[3])
}


