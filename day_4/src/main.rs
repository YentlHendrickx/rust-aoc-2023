fn main() {
    let input = std::fs::read_to_string("./data/input.txt").unwrap();
    let vectorized: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let total_points: u32 = first_half(&vectorized);

    println!("Total points: {}", total_points);

    let total_cards: u32 = second_half(&vectorized);
    println!("Total cards: {}", total_cards);
}

fn second_half(vectorized: &Vec<&str>) -> u32 {
    let mut result = 0;
    for (i, line) in vectorized.iter().enumerate() {
        result += mutate_line(line, vectorized, (i as u32));
    }

    return result;
}

fn mutate_line(line: &str, vectorized: &Vec<&str>, current: u32) -> u32 {
    let mut card_count = 0;
    let mut result = 1;
    let colon_index = line.find(':').unwrap();
    let stripped_line = &line[colon_index + 2..].trim();

    let left_numbers = stripped_line.split('|').collect::<Vec<&str>>()[0];
    let right_numbers = stripped_line.split('|').collect::<Vec<&str>>()[1];

    let mut winning_numbers = Vec::new();

    for number in left_numbers.trim().split(' ').collect::<Vec<&str>>() {
        if number.is_empty() {
            continue;
        }
        winning_numbers.push(number);
    }

    for number in right_numbers.trim().split(' ').collect::<Vec<&str>>() {
        if number.is_empty() {
            continue;
        }

        if winning_numbers.contains(&number) {
            // Convert number to u32
            card_count += 1;
            // println!("Current {}, Result {}", current, card_count);
        }
    }

    for number in 1..card_count + 1 {
        let new_line_number = number + current;
        // println!("New line number: {}", new_line_number);
        if new_line_number >= vectorized.len() as u32 {
            return result;
        }

        let new_line = &vectorized[new_line_number as usize..(new_line_number + 1) as usize][0];
        result += mutate_line(new_line, vectorized, new_line_number);
    }

    return result;
}

fn first_half(vectorized: &Vec<&str>) -> u32 {
    let mut total_points = 0;

    for (i, line) in vectorized.iter().enumerate() {
        let colon_index = line.find(':').unwrap();
        let stripped_line = &line[colon_index + 2..].trim();

        let left_numbers = stripped_line.split('|').collect::<Vec<&str>>()[0];
        let right_numbers = stripped_line.split('|').collect::<Vec<&str>>()[1];

        let mut winning_numbers = Vec::new();

        for number in left_numbers.trim().split(' ').collect::<Vec<&str>>() {
            if number.is_empty() {
                continue;
            }
            winning_numbers.push(number);
        }

        let mut points = 0;

        for number in right_numbers.trim().split(' ').collect::<Vec<&str>>() {
            if number.is_empty() {
                continue;
            }

            if winning_numbers.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        total_points += points;
    }

    return total_points;
}
