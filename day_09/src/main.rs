use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let part_1 = solve(&input, false);
    println!("Part 1: {}", part_1);

    let part_2 = solve(&input, true);
    println!("Part 2: {}", part_2)
}

fn solve(input: &str, part_two: bool) -> f64 {
    let mut result: f64 = 0.0;

    let histories = parse_input(input);

    for history in histories {
        let new_value = extrapolate_data(&history, part_two);

        result += new_value;
    }

    return result;
}

// is_previous: true = previous, false = next
fn extrapolate_data(history: &[f64], is_previous: bool) -> f64 {
    let start = if is_previous { 0 } else { 1 };
    let end = if is_previous { 1 } else { 0 };

    let mut current_difference = history
        .windows(2)
        .map(|w| w[start] - w[end])
        .collect::<Vec<_>>();

    let mut differences: Vec<f64> = vec![
        current_difference.clone()[if is_previous {
            0
        } else {
            current_difference.len() - 1
        }],
    ];

    while !current_difference.iter().all(|&x| x == 0.0) {
        current_difference = current_difference
            .windows(2)
            .map(|w| w[start] - w[end])
            .collect();

        differences.push(
            current_difference.clone()[if is_previous {
                0
            } else {
                current_difference.len() - 1
            }],
        );
    }

    let mut result = history[if is_previous { 0 } else { history.len() - 1 }];
    for difference in differences {
        result += difference;
    }
    return result;
}

fn parse_input(input: &str) -> Vec<Vec<f64>> {
    return input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<f64>().unwrap())
                .collect()
        })
        .collect();
}
