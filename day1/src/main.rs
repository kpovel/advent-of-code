use std::{env, fs};

fn main() {
    let envs: Vec<String> = env::args().collect();

    let file_path = &envs[1];

    let file_content = read_file(file_path);
    let elves_foods = split_elves_foods(&file_content);
    let calories = sum_calories(&elves_foods);
    show_result(&calories);
}

fn read_file(path: &str) -> String {
    return fs::read_to_string(path).expect("Just read it");
}

fn split_elves_foods(fc: &str) -> Vec<String> {
    fc.split("\n\n").map(|s| s.to_owned()).collect()
}

fn sum_calories(elves_foods: &Vec<String>) -> Vec<u32> {
    let mut calories_sum = vec![];

    for e in elves_foods {
        let sum = e
            .split("\n")
            .map(|cal| cal.parse().unwrap_or(0))
            .reduce(|f, s| f + s);
        match sum {
            Some(x) => calories_sum.push(x),
            None => calories_sum.push(0),
        }
    }

    calories_sum
}

fn show_result(calories_list: &Vec<u32>) {
    match calories_list.iter().max() {
        Some(x) => println!("The most calories is {}", x),
        None => println!("No calories"),
    }
}
