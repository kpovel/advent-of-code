use std::fs;

fn main() {
    let first_part = solve_first_part();
    println!("First part: {}", first_part);
}

fn solve_first_part() -> u32 {
    let file_content = fs::read_to_string("input").unwrap();

    let mut fully_contains = 0;

    for l in file_content.trim().split("\n").into_iter() {
        let nums: Vec<u32> = l
            .split(&['-', ',', '-'][..])
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let (first, second, third, fourth) = (nums[0], nums[1], nums[2], nums[3]);

        match (first <= third, second >= fourth) {
            (true, true) => {
                fully_contains += 1;
                continue;
            }
            _ => (),
        };

        match (first >= third, second <= fourth) {
            (true, true) => fully_contains += 1,
            _ => (),
        };
    }

    fully_contains
}
