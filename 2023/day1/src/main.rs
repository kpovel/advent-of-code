fn main() {
    let input1 = include_str!("./input1");
    let first_part_res = solve_first_part(&input1);
    println!("The result of the first part is {}", first_part_res);

    let input2 = include_str!("./input2");
    let second_part_res = solve_second_part(&input2);
    println!("The result of the second part is {}", second_part_res);
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let calibration_arr: Vec<_> = l.chars().filter(|c| c.is_digit(10)).collect();

            let combined_values = format!(
                "{}{}",
                calibration_arr.first().expect("not empty vec"),
                calibration_arr.last().expect("not empty vec")
            );

            combined_values.parse::<u32>().expect("number")
        })
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let digits = find_digits(&l);
            let calibration_values = format!(
                "{}{}",
                digits.first().expect("not empty vec"),
                digits.last().expect("not empty vec")
            );

            calibration_values.parse::<u32>().expect("number")
        })
        .sum()
}

fn find_digits(line: &str) -> Vec<u32> {
    let mut digit_str = vec![];
    let calibration_symbols = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in 0..line.len() {
        calibration_symbols.iter().for_each(|symbol| {
            if line[i..].starts_with(symbol) {
                match *symbol {
                    "one" => digit_str.push(1),
                    "two" => digit_str.push(2),
                    "three" => digit_str.push(3),
                    "four" => digit_str.push(4),
                    "five" => digit_str.push(5),
                    "six" => digit_str.push(6),
                    "seven" => digit_str.push(7),
                    "eight" => digit_str.push(8),
                    "nine" => digit_str.push(9),
                    _ => (),
                };
            } else if let Ok(num) = line[i..i + 1].parse::<u32>() {
                digit_str.push(num);
            }
        })
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
