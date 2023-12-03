#![feature(let_chains)]

fn main() {
    let input1 = include_str!("input1");
    println!("First part: {}", first_part(&input1));
}

#[derive(Debug)]
struct EnginePart {
    part: u32,
    start_position: usize,
    end_postion: usize,
}

fn first_part(input: &str) -> u32 {
    let split_input = input.lines().collect::<Vec<_>>();

    split_input
        .iter()
        .map(|line| {
            let mut engine_parts = vec![];
            let mut part_start = None;
            let mut part_value = 0;

            line.chars().enumerate().for_each(|(position, char)| {
                let parsed_char = char.to_digit(10) as Option<u32>;

                if part_start.is_none() && parsed_char.is_some() {
                    part_start = Some(position);
                }

                if let Some(num) = parsed_char {
                    part_value = part_value * 10 + num;
                }

                if let Some(start) = part_start
                    && parsed_char.is_none()
                {
                    engine_parts.push(EnginePart {
                        part: part_value,
                        start_position: start,
                        end_postion: position,
                    });

                    part_value = 0;
                    part_start = None;
                } else if let Some(start) = part_start
                    && position == line.len() - 1
                {
                    if parsed_char.is_none() {
                        engine_parts.push(EnginePart {
                            part: part_value,
                            start_position: start,
                            end_postion: position,
                        });
                    } else {
                        engine_parts.push(EnginePart {
                            part: part_value,
                            start_position: start,
                            end_postion: position + 1,
                        });
                    }
                }
            });

            engine_parts
        })
        .enumerate()
        .map(|(line_number, nums)| {
            nums.into_iter()
                .filter_map(|n| {
                    let is_wall_top = line_number == 0;
                    let is_wall_right = n.end_postion == split_input[line_number].len();
                    let is_wall_buttom = line_number == split_input.len() - 1;
                    let is_wall_left = n.start_position == 0;
                    let is_adjacent_symbol = |s: &str| {
                        if let Ok(_) = s.parse::<u32>() {
                            false
                        } else if s == "." {
                            false
                        } else {
                            true
                        }
                    };

                    if is_wall_top && is_wall_left {
                        for i in n.start_position..n.end_postion + 1 {
                            let buttom_symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(buttom_symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_top && is_wall_right {
                        for i in n.start_position - 1..n.end_postion {
                            let top_symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(top_symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_top {
                        for i in n.start_position - 1..n.end_postion + 1 {
                            let symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) || is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_buttom && is_wall_left {
                        for i in n.start_position..n.end_postion + 1 {
                            let symbol = &split_input[line_number - 1][i..i + 1];
                            if is_adjacent_symbol(symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_buttom && is_wall_right {
                        for i in n.start_position - 1..n.end_postion {
                            let symbol = &split_input[line_number - 1][i..i + 1];
                            if is_adjacent_symbol(symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_buttom {
                        for i in n.start_position - 1..n.end_postion + 1 {
                            let symbol = &split_input[line_number - 1][i..i + 1];
                            if is_adjacent_symbol(symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) || is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_left {
                        for i in n.start_position..n.end_postion + 1 {
                            let top_symbol = &split_input[line_number - 1][i..i + 1];
                            let buttom_symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(top_symbol) || is_adjacent_symbol(buttom_symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) {
                            return Some(n);
                        }

                        None
                    } else if is_wall_right {
                        for i in n.start_position - 1..n.end_postion {
                            let top_symbol = &split_input[line_number - 1][i..i + 1];
                            let buttom_symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(top_symbol) || is_adjacent_symbol(buttom_symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

                        None
                    } else {
                        for i in n.start_position - 1..n.end_postion + 1 {
                            let top_symbol = &split_input[line_number - 1][i..i + 1];
                            let buttom_symbol = &split_input[line_number + 1][i..i + 1];
                            if is_adjacent_symbol(top_symbol) || is_adjacent_symbol(buttom_symbol) {
                                return Some(n);
                            }
                        }

                        if is_adjacent_symbol(
                            &split_input[line_number][n.end_postion..n.end_postion + 1],
                        ) || is_adjacent_symbol(
                            &split_input[line_number][n.start_position - 1..n.start_position],
                        ) {
                            return Some(n);
                        }

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
