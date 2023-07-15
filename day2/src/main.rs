use std::fs;

enum GameElements {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let file_content = fs::read_to_string("./input").expect("file");
    let game_result = start_the_game(&file_content);
    println!("Result: {}", game_result);
}

fn start_the_game(input: &str) -> u32 {
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
