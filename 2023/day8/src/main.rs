use std::collections::HashMap;

fn main() {
    let input1 = include_str!("./input1");
    let first_part_result = first_part(&input1);
    println!("First part: {}", first_part_result);
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

#[cfg(test)]
mod test {
    use crate::first_part;

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
}
