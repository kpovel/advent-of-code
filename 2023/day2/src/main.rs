fn main() {
    let input1 = include_str!("./input1");
    let first_part_result = solve_first_part(&input1);
    println!("The result of the first part is {}", first_part_result);
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let game_id = l.split(":").collect::<Vec<_>>()[0]
                .split(" ")
                .collect::<Vec<_>>()[1]
                .parse::<u32>()
                .expect("game id");

            let is_impossible_game = l.split(":").collect::<Vec<_>>()[1]
                .split(";")
                .map(|cube_set| {
                    cube_set.split(",").any(|cube| {
                        let cube = cube.trim().split(" ").collect::<Vec<_>>();
                        let (cube_number, color) =
                            (cube[0].parse::<u8>().expect("number of cubes"), cube[1]);

                        match color {
                            "red" => cube_number > 12,
                            "green" => cube_number > 13,
                            "blue" => cube_number > 14,
                            _ => panic!("unknown cube color"),
                        }
                    })
                })
                .any(|i| i);

            if is_impossible_game {
                None
            } else {
                Some(game_id)
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solve_first_part;

    #[test]
    fn first_part() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_first_part(&input), 8);
    }
}
