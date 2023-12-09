use std::collections::HashMap;

fn main() {
    let input1 = include_str!("./input1");
    let first_part_result = first_part(&input1);
    println!("First part: {}", first_part_result);

    let second_part_result = second_part(&input1);
    println!("Second part: {}", second_part_result);
}

fn first_part(input: &str) -> u32 {
    let splitted_map = input.split("\n\n").collect::<Vec<_>>();
    let navigation_instuction = splitted_map[0];
    let navigation_network = splitted_map[1]
        .lines()
        .fold(HashMap::new(), |mut nodes, l| {
            let splitted_position = l
                .split(&['=', '(', ',', ')', ' '][..])
                .filter(|&e| e != "")
                .collect::<Vec<_>>();

            nodes.insert(
                splitted_position[0],
                (splitted_position[1], splitted_position[2]),
            );

            nodes
        });

    let mut steps_to_reach_finish = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let route_index = steps_to_reach_finish % navigation_instuction.len();
        let route_path = &navigation_instuction[route_index..route_index + 1];

        current_node = match route_path {
            "L" => navigation_network.get(current_node).unwrap().0,
            "R" => navigation_network.get(current_node).unwrap().1,
            _ => unreachable!(),
        };

        steps_to_reach_finish += 1;
    }

    steps_to_reach_finish as u32
}

fn second_part(input: &str) -> u64 {
    let splitted_map = input.split("\n\n").collect::<Vec<_>>();
    let navigation_instuction = splitted_map[0];
    let navigation_network = splitted_map[1]
        .lines()
        .fold(HashMap::new(), |mut nodes, l| {
            let splitted_position = l
                .split(&['=', '(', ',', ')', ' '][..])
                .filter(|&e| e != "")
                .collect::<Vec<_>>();

            nodes.insert(
                splitted_position[0],
                (splitted_position[1], splitted_position[2]),
            );

            nodes
        });

    let path_lengths = navigation_network
        .iter()
        .filter(|path| path.0.ends_with("A"))
        .map(|n| {
            let mut i = 0;
            let mut current = n;

            while !current.0.ends_with("Z") {
                let route_index = i % navigation_instuction.len();
                let route_path = &navigation_instuction[route_index..route_index + 1];

                current = match route_path {
                    "L" => navigation_network.get_key_value(current.1 .0).unwrap(),
                    "R" => navigation_network.get_key_value(current.1 .1).unwrap(),
                    _ => unreachable!(),
                };

                i += 1;
            }

            i as u64
        })
        .collect::<Vec<_>>();


        lcm_of_vector(&path_lengths)
}

fn lcm_of_vector(numbers: &Vec<u64>) -> u64 {
    numbers.iter().fold(1, |lcm, &num| lcm_of_two(lcm, num))
}

fn lcm_of_two(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b // LCM formula using GCD
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod test {
    use crate::{first_part, second_part};

    #[test]
    fn test_first_part() {
        let input1 = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input2 = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(first_part(&input1), 2);
        assert_eq!(first_part(&input2), 6);
    }

    #[test]
    fn test_second_part() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(second_part(&input), 6);
    }
}
