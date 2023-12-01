fn main() {
    let input1 = include_str!("./input1");
    let first_part_res = solve_first_part(&input1);
    println!("The result of the first part is {}", first_part_res);
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
}
