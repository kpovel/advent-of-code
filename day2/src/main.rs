use std::fs;

enum GameElements {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let file_content = fs::read_to_string("./input").expect("file");
    let first_half_result = calculate_first_half(&file_content);
    println!("Result: {}", first_half_result);
    let second_half_result = calculate_second_half(&file_content);
    println!("Second half: {}", second_half_result);
}

fn calculate_second_half(input: &str) -> u32 {
    let mut count = 0;
    for line in input.trim().lines() {
        let choose: Vec<&str> = line.split_whitespace().collect();
        let (opponent_choose, my_choose) = match_choose(choose[0], choose[1]);

        match (opponent_choose, my_choose) {
            (GameElements::Rock, GameElements::Rock) => count += 3,
            (GameElements::Paper, GameElements::Rock) => count += 1,
            (GameElements::Scissors, GameElements::Rock) => count += 2,

            (GameElements::Rock, GameElements::Paper) => count  += 4,
            (GameElements::Paper, GameElements::Paper) => count += 5,
            (GameElements::Scissors, GameElements::Paper) => count += 6,

            (GameElements::Rock, GameElements::Scissors) => count += 8,
            (GameElements::Paper, GameElements::Scissors) => count += 9,
            (GameElements::Scissors, GameElements::Scissors) => count += 7,
        }
    }

    count
}

fn calculate_first_half(input: &str) -> u32 {
    let mut count = 0u32;
    for line in input.trim().lines() {
        let choose: Vec<&str> = line.split_whitespace().collect();
        let (opponent_choose, my_choose) = match_choose(choose[0], choose[1]);

        match (&opponent_choose, &my_choose) {
            (GameElements::Rock, GameElements::Paper) => count += 6,
            (GameElements::Rock, GameElements::Scissors) => count += 0,
            (GameElements::Paper, GameElements::Rock) => count += 0,
            (GameElements::Paper, GameElements::Scissors) => count += 6,
            (GameElements::Scissors, GameElements::Rock) => count += 6,
            (GameElements::Scissors, GameElements::Paper) => count += 0,
            _ => count += 3,
        };

        match &my_choose {
            GameElements::Rock => count += 1,
            GameElements::Paper => count += 2,
            GameElements::Scissors => count += 3,
        };
    }

    count
}

fn match_choose(opponent_choose: &str, my_choose: &str) -> (GameElements, GameElements) {
    let opponent = match opponent_choose {
        "A" => GameElements::Rock,
        "B" => GameElements::Paper,
        _ => GameElements::Scissors,
    };

    let me = match my_choose {
        "X" => GameElements::Rock,
        "Y" => GameElements::Paper,
        _ => GameElements::Scissors,
    };

    (opponent, me)
}
