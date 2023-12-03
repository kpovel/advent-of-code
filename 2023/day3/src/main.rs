#![feature(let_chains)]

use std::ops::Range;

fn main() {
    let input1 = include_str!("input1");
    println!("First part: {}", first_part(&input1));
}

#[derive(Debug)]
struct EnginePart {
    part: u32,
    start_position: usize,
    end_position: usize,
}

fn check_adjacent_symbols(position_range: Range<usize>, line: &str) -> bool {
    position_range
        .map(|i| &line[i..i + 1])
        .any(|s| !s.parse::<u32>().is_ok() && s != ".")
}

fn first_part(input: &str) -> u32 {
    let split_input = input.lines().collect::<Vec<_>>();

    split_input
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .fold(
                    (vec![], None, 0),
                    |(mut engine_parts, part_start, mut part_value), (position, char)| match char
                        .to_digit(10)
                    {
                        Some(num) => {
                            part_value = part_value * 10 + num;

                            if position == line.len() - 1 {
                                engine_parts.push(EnginePart {
                                    part: part_value,
                                    start_position: part_start.unwrap(),
                                    end_position: position + 1,
                                })
                            }

                            (engine_parts, part_start.or(Some(position)), part_value)
                        }
                        None => {
                            if let Some(start) = part_start {
                                engine_parts.push(EnginePart {
                                    part: part_value,
                                    start_position: start,
                                    end_position: position,
                                });
                            }
                            (engine_parts, None, 0)
                        }
                    },
                )
                .0
        })
        .enumerate()
        .map(|(line_number, nums)| {
            nums.into_iter()
                .filter_map(|n| {
                    let is_wall_top = line_number == 0;
                    let is_wall_right = n.end_position == split_input[line_number].len();
                    let is_wall_bottom = line_number == split_input.len() - 1;
                    let is_wall_left = n.start_position == 0;

                    let horizontal_range = match (is_wall_left, is_wall_right) {
                        (true, true) => n.start_position..n.end_position,
                        (true, false) => n.start_position..n.end_position + 1,
                        (false, true) => n.start_position - 1..n.end_position,
                        (false, false) => n.start_position - 1..n.end_position + 1,
                    };
                    let left_range = if is_wall_left {
                        0..0
                    } else {
                        n.start_position - 1..n.end_position
                    };
                    let right_range = if is_wall_left {
                        n.end_position..n.end_position + 1
                    } else {
                        n.end_position..n.end_position + 1
                    };

                    let adjacent_top = !is_wall_top
                        && check_adjacent_symbols(
                            horizontal_range.clone(),
                            &split_input[line_number - 1],
                        );
                    let adjacent_bottom = !is_wall_bottom
                        && check_adjacent_symbols(horizontal_range, &split_input[line_number + 1]);
                    let adjacent_left = !is_wall_left
                        && check_adjacent_symbols(left_range, &split_input[line_number]);
                    let adjacent_right = !is_wall_right
                        && check_adjacent_symbols(right_range, &split_input[line_number]);

                    if adjacent_top || adjacent_bottom || adjacent_left || adjacent_right {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|n| {
            n.iter()
                .map(|n| n.part)
                .collect::<Vec<_>>()
                .iter()
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::first_part;

    #[test]
    fn test_first_part() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(first_part(&input), 4361);
    }
}
