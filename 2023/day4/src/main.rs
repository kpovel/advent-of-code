fn main() {
    let input1 = include_str!("./input1");
    let first_part_res = first_part(&input1);
    println!("First part: {}", first_part_res);

    let second_part_res = second_part(&input1);
    println!("First part: {}", second_part_res);
}

fn first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let numbers = l.trim().split(":").collect::<Vec<_>>()[1]
                .split("|")
                .map(|s| {
                    let res = s
                        .split_whitespace()
                        .map(|n| n.parse::<u32>().expect("number"))
                        .collect::<Vec<_>>();
                    res
                })
                .collect::<Vec<_>>();

            let (winning_number, my_numbers) = (&numbers[0], &numbers[1]);

            my_numbers
                .iter()
                .filter(|n| winning_number.contains(n))
                .fold(
                    0u32,
                    |total_win, _d| {
                        if total_win == 0 {
                            1
                        } else {
                            total_win * 2
                        }
                    },
                )
        })
        .sum()
}

fn won_cards(cards: &str, curr_card: u32) -> u32 {
    let binding = cards.lines().collect::<Vec<_>>();
    let card = &binding.get(curr_card as usize - 1);

    match card {
        Some(&card) => {
            let card_numbers = card.split(":").collect::<Vec<_>>()[1]
                .split("|")
                .map(|p| {
                    p.split_whitespace()
                        .map(|n| n.parse::<u32>().expect("number"))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let (winning_numbers, my_numbers) = (&card_numbers[0], &card_numbers[1]);

            let matching_numbers = my_numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count() as u32;

            let card_range = curr_card + 1..curr_card + matching_numbers + 1;

            card_range.fold(1, |scratchcards, what| {
                won_cards(cards, what) + scratchcards
            })
        }
        None => 1,
    }
}

fn second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let card_number = l.split(":").collect::<Vec<_>>()[0]
                .split_whitespace()
                .collect::<Vec<_>>()[1]
                .parse::<u32>()
                .expect("card number");

            won_cards(input, card_number)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(first_part(&input), 13);
    }

    #[test]
    fn test_second_part() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(second_part(&input), 30);
    }
}
