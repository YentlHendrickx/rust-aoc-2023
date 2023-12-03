use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./data/input.txt").unwrap();
    let vectorized: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let sum_of_parts: u32 = first_half(&vectorized);

    println!("Sum is {}", sum_of_parts);

    let sum_of_gears: u32 = second_half(&vectorized);
    println!("Sum of gears {}", sum_of_gears);
}

fn second_half(vectorized: &Vec<&str>) -> u32 {
    let mut sum_of_gears: u32 = 0;

    let mut gear_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    for (line_index, line) in vectorized.iter().enumerate() {
        let mut current_number = String::new();
        let mut start_index: u32 = 0;

        for (c_index, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                if current_number.len() == 0 {
                    start_index = c_index as u32;
                }

                current_number.push_str(&c.to_digit(10).unwrap().to_string());
            } else {
                if current_number.len() > 0 {
                    let result = check_gear(vectorized, line_index, &current_number, start_index);

                    for (x, y, number) in result {
                        // Retrieve the vector associated with the (x, y) key, or create a new one if it doesn't exist
                        let entry = gear_map.entry((x, y)).or_insert_with(Vec::new);
                        // Add the number to the vector
                        entry.push(number);
                    }

                    start_index = 0;
                    current_number = String::new();
                }
            }
        }

        if current_number.len() > 0 {
            let result = check_gear(vectorized, line_index, &current_number, start_index);

            for (x, y, number) in result {
                // Retrieve the vector associated with the (x, y) key, or create a new one if it doesn't exist
                let entry = gear_map.entry((x, y)).or_insert_with(Vec::new);
                // Add the number to the vector
                entry.push(number);
            }
        }
    }

    // Loop over hashmap, find coordinated having only 2 matches, add numbers
    for ((x, y), numbers) in &gear_map {
        if numbers.len() == 2 {
            // Assuming numbers[0] and numbers[1] are the two elements you want to use
            let number1 = numbers[0];
            let number2 = numbers[1];

            sum_of_gears += number1 * number2;
        }
    }

    return sum_of_gears;
}

fn check_gear(lines: &Vec<&str>, index: usize, number: &str, start: u32) -> Vec<(u32, u32, u32)> {
    let start_index_usize = start as usize;

    // Go from top left to top right
    let mut left_bound: usize = 0;
    let mut right_bound: usize = start_index_usize + number.len() + 1;

    if start_index_usize > 1 {
        left_bound = start_index_usize - 1;
    }

    if (start_index_usize + number.len()) > lines.len() {
        right_bound = lines.len() + 2;
    }

    let left_x = left_bound;
    let right_x = right_bound;

    let mut top_y = index;
    let mut bot_y = index + 1;

    if index > 0 {
        top_y -= 1;
    }

    if index < lines.len() {
        bot_y += 1;
    }

    if bot_y > lines.len() {
        bot_y = lines.len();
    }

    println!("{} {}", top_y, bot_y);

    let mut found_gears: Vec<(u32, u32, u32)> = Vec::new();

    for y in top_y..bot_y {
        for x in left_x..right_x as usize {
            println!("{}, {}", x, y);
            let char = lines[y].chars().nth(x);

            if let Some(c) = char {
                if c == '*' {
                    found_gears.push((x as u32, y as u32, number.parse::<u32>().unwrap()));
                }
            }
        }
    }

    println!("\n");
    return found_gears;
}

fn first_half(vectorized: &Vec<&str>) -> u32 {
    let mut sum_of_parts: u32 = 0;

    for (line_index, line) in vectorized.iter().enumerate() {
        let mut current_number = String::new();
        let mut start_index: u32 = 0;

        for (c_index, c) in line.trim().chars().enumerate() {
            if c.is_digit(10) {
                if current_number.len() == 0 {
                    start_index = c_index as u32;
                }

                current_number.push_str(&c.to_digit(10).unwrap().to_string());
            } else {
                if current_number.len() > 0 {
                    let result = check_number(vectorized, line_index, &current_number, start_index);

                    if result {
                        sum_of_parts += current_number.parse::<u32>().unwrap();
                    }

                    start_index = 0;
                    current_number = String::new();
                }
            }
        }

        if current_number.len() > 0 {
            let result = check_number(vectorized, line_index, &current_number, start_index);

            if result {
                sum_of_parts += current_number.parse::<u32>().unwrap();
            }
        }
    }

    return sum_of_parts;
}

fn check_number(lines: &Vec<&str>, current_line: usize, number: &str, start_index: u32) -> bool {
    // Check top left of number

    let current_line_usize = current_line as usize;
    let start_index_usize = start_index as usize;

    // Go from top left to top right
    let mut left_bound: usize = 0;
    let mut right_bound: usize = start_index_usize + number.len() + 1;

    if start_index_usize > 1 {
        left_bound = start_index_usize - 1;
    }

    if (start_index_usize + number.len()) > lines.len() {
        right_bound = lines.len() + 2;
    }

    if is_part_number(lines[current_line].trim(), left_bound, right_bound) {
        return true;
    }

    if current_line_usize > 0 {
        let line_above = lines[current_line_usize - 1].trim();

        if is_part_number(line_above, left_bound, right_bound) {
            if number == "278" {
                println!("AYO2?");
            }
            return true;
        }
    }

    if current_line_usize < lines.len() - 1 {
        let line_below = lines[current_line_usize + 1].trim();

        if is_part_number(line_below, left_bound, right_bound) {
            return true;
        }
    }

    return false;
}

fn is_part_number(line: &str, left: usize, right: usize) -> bool {
    for index in left..right {
        let char = line.chars().nth(index);

        if let Some(c) = char {
            if c != '.' && !c.is_digit(10) {
                return true;
            }
        }
    }

    return false;
}
