use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../data/input.txt").unwrap();
    let vectorized: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let total: u32 = first_half(&vectorized);
    println!("Total: {}", total);

    let total: u32 = second_half(&vectorized);
    println!("Total: {}", total);
}

fn second_half(vectorized: &Vec<&str>) -> u32 {
    let mut total: u32 = 0;

    let list = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in vectorized.iter() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut found_first: bool = false;

        for (index, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if !found_first {
                    first = c.to_digit(10).unwrap();
                    found_first = true;
                }
                last = c.to_digit(10).unwrap();
            }

            for (key, value) in list.iter() {
                if index + key.len() <= line.len() {
                    let slice = &line[index..index + key.len()];
                    if &slice == key {
                        if !found_first {
                            found_first = true;
                            first = *value;
                        }

                        last = *value;
                    }
                }
            }
        }

        let combination: u32 = (first * 10) + last;
        total += combination;
    }

    return total;
}

fn first_half(vectorized: &Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in vectorized.iter() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut found_first: bool = false;

        for c in line.chars() {
            if c.is_digit(10) {
                if !found_first {
                    first = c.to_digit(10).unwrap();
                    found_first = true;
                }
                last = c.to_digit(10).unwrap();
            }
        }

        let combination: u32 = (first * 10) + last;
        total += combination;
    }

    return total;
}
