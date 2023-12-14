use std::fs;

mod backup;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let parsed = parse_input(&input);
    let result = solve(&parsed, false);
    println!("part 1: {}", result);

    let result = solve(&parsed, true);
    println!("part 2: {}", result);
}

fn solve(patterns: &Vec<Vec<String>>, part_two: bool) -> u64 {
    let mut total = 0;

    for (_i, pattern) in patterns.iter().enumerate() {
        for orientation in ['v', 'h'].iter() {
            let mut current_pattern = pattern.clone();
            if *orientation == 'v' {
                let mut new_pattern = Vec::new();
                for i in 0..current_pattern[0].len() {
                    let mut new_row = String::new();
                    for j in 0..current_pattern.len() {
                        new_row.push(current_pattern[j].chars().nth(i).unwrap());
                    }
                    new_pattern.push(new_row);
                }

                current_pattern = new_pattern;
            }

            let mirror_index = find_horizontal_mirror_index(&current_pattern, part_two);

            if mirror_index != 0 {
                if *orientation == 'v' {
                    total += mirror_index;
                } else {
                    total += 100 * mirror_index;
                }

                break;
            }
        }
    }

    return total;
}

fn find_horizontal_mirror_index(pattern: &[String], part_two: bool) -> u64 {
    for i in 0..pattern.len() - 1 {
        if reflects(pattern, i, part_two) {
            return i as u64 + 1;
        }
    }

    return 0;
}

fn reflects(pattern: &[String], axis: usize, part_two: bool) -> bool {
    let mut ptr1 = axis;
    let mut ptr2 = axis + 1;

    let mut mistakes = 0;

    loop {
        if pattern[ptr1] != pattern[ptr2] {
            if !part_two {
                return false;
            }

            // Loop and check mistakes
            for (i, chars) in pattern[ptr1].chars().enumerate() {
                if pattern[ptr2].chars().nth(i).unwrap() != chars {
                    mistakes += 1;
                }
            }
        }
        if ptr1 == 0 || ptr2 == pattern.len() - 1 {
            break;
        }
        ptr1 -= 1;
        ptr2 += 1;
    }

    if part_two && mistakes == 1 {
        return true;
    } else if part_two {
        return false;
    }

    return true;
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    input
        .split("\r\n\r\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}
