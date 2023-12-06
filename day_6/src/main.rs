use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let part_1 = solve(&input, false);
    println!("Part 1: {}", part_1);

    let part_2 = solve(&input, true);
    println!("Part 2: {}", part_2);
}

fn solve(input: &str, part_two: bool) -> u64 {
    let (times, distances) = extract_input(input, part_two);

    // Loop over all times
    let mut total_product = 1;
    for (i, time) in times.iter().enumerate() {
        let mut win_count = 0;
        for value in 1..*time {
            let distance = value * (time - value);
            if distance > distances[i] {
                win_count += 1;
            }
        }

        total_product *= win_count;
    }

    return total_product;
}

fn extract_input(input: &str, part_two: bool) -> (Vec<u64>, Vec<u64>) {
    let times = input
        .lines()
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // If part two, loop over all times and combine into string, then parse back into u32
    // Probably better way but honestly for such a small input it's not worth it
    if part_two {
        let mut new_time = String::new();
        for (_i, time) in times.iter().enumerate() {
            new_time.push_str(&time.to_string());
        }

        let mut new_distance = String::new();
        for (_i, distance) in distances.iter().enumerate() {
            new_distance.push_str(&distance.to_string());
        }

        return (
            vec![new_time.parse::<u64>().unwrap()],
            vec![new_distance.parse::<u64>().unwrap()],
        );
    }

    return (times, distances);
}
