fn main() {
    let input1 = include_str!("./input1");
    let first_part_res = solve_first_part(&input1);
    println!("The result of the first part is {}", first_part_res);

    let input2 = include_str!("./input2");
    let second_part_res = solve_second_part(&input2);
    println!("The result of the second part is {}", second_part_res);
}

fn solve_first_part(input: &str) -> u32 {
    let mut calibration_sum = 0;

    for l in input.lines() {
        let calibration_arr: Vec<_> = l
            .split("")
            .filter(|c| match c.parse::<u32>() {
                Ok(_) => true,
                Err(_) => false,
            })
            .collect();

        let compbined_values = format!(
            "{}{}",
            calibration_arr.first().expect("not empty vec"),
            calibration_arr.last().expect("not empty vec")
        );

        calibration_sum += compbined_values.parse::<u32>().expect("number");
    }

    calibration_sum
}

fn solve_second_part(input: &str) -> u32 {
    let mut calibration_sum = 0;

    for l in input.lines() {
        let digits = find_digits(&l);
        let calibration_values = format!(
            "{}{}",
            digits.first().expect("not empty vec"),
            digits.last().expect("not empty vec")
        );

        calibration_sum += calibration_values.parse::<u32>().expect("number");
    }

    calibration_sum
}

fn find_digits(line: &str) -> Vec<&str> {
    let mut digit_str = vec![];
    let calibration_symbols = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in 0..line.len() {
        for symbol in calibration_symbols.iter() {
            if line[i..].starts_with(symbol) {
                match *symbol {
                    "one" => digit_str.push("1"),
                    "two" => digit_str.push("2"),
                    "three" => digit_str.push("3"),
                    "four" => digit_str.push("4"),
                    "five" => digit_str.push("5"),
                    "six" => digit_str.push("6"),
                    "seven" => digit_str.push("7"),
                    "eight" => digit_str.push("8"),
                    "nine" => digit_str.push("9"),
                    _ => (),
                };
            } else if let Ok(_) = line[i..i + 1].parse::<u32>() {
                digit_str.push(&line[i..i + 1]);
            }
        }
    }

    digit_str
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_part() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_first_part(&input), 142);
    }

    #[test]
    fn second_part() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_second_part(&input), 281);
    }
}
