fn main() {
    let input1 = include_str!("./input1");
    let first_part_result = first_part(&input1);
    println!("First part: {}", first_part_result);
}

fn first_part(input: &str) -> u32 {
    let records = input
        .lines()
        .map(|l| {
            l.split(":").collect::<Vec<_>>()[1]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (times, distances) = (&records[0], &records[1]);

    times
        .iter()
        .enumerate()
        .map(|(i, &time)| {
            let record_distance = distances[i];

            let range = 1..time;
            range.fold(0u32, |win_rounds, speed| {
                let remign_distance = time - speed;
                let pass_distance = remign_distance * speed;

                if pass_distance > record_distance {
                    win_rounds + 1
                } else {
                    win_rounds
                }
            })
        })
        .reduce(|acc, e| acc * e)
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::first_part;

    #[test]
    fn test_first_part() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(first_part(&input), 288);
    }
}
