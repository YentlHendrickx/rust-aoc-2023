fn main() {
    let input = std::fs::read_to_string("./data/input.txt").unwrap();
    let vectorized: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let sum_of_ids: u32 = first_half(&vectorized, 12, 13, 14);
    let sum_of_powers: u32 = second_half(&vectorized);

    println!("Sum of id: {}", sum_of_ids);
    println!("Sum of power: {}", sum_of_powers);
}

fn second_half(vectorized: &Vec<&str>) -> u32 {
    let mut sum_of_powers: u32 = 0;

    // Loop over all the lines in the input file
    for game in vectorized.iter() {
        println!("{}", game);

        let (game_id, colon_index) = get_game_id(game);
        println!("Game ID: {}", game_id);

        let mut red_min: u32 = 0;
        let mut green_min: u32 = 0;
        let mut blue_min: u32 = 0;

        let rounds: Vec<&str> = (&game[colon_index as usize + 2..]).split(';').collect();

        for round in rounds.iter() {
            println!("Round: {}", round.trim());
            let (red, green, blue) = get_round_totals(round.trim());

            if red > red_min {
                red_min = red;
            }

            if green > green_min {
                green_min = green;
            }

            if blue > blue_min {
                blue_min = blue;
            }
        }

        let power: u32 = green_min * red_min * blue_min;

        sum_of_powers += power;
    }

    return sum_of_powers;
}

fn first_half(vectorized: &Vec<&str>, r_t: u32, g_t: u32, b_t: u32) -> u32 {
    let mut sum_of_ids: u32 = 0;

    // Loop over all the lines in the input file
    for game in vectorized.iter() {
        println!("{}", game);

        let (game_id, colon_index) = get_game_id(game);
        println!("Game ID: {}", game_id);

        let mut possible: bool = true;

        let rounds: Vec<&str> = (&game[colon_index as usize + 2..]).split(';').collect();

        for round in rounds.iter() {
            println!("Round: {}", round.trim());
            let (red, green, blue) = get_round_totals(round.trim());

            if red > r_t || green > g_t || blue > b_t {
                possible = false;
                break;
            }
        }

        if possible {
            sum_of_ids += game_id;
        }
    }

    return sum_of_ids;
}

fn get_round_totals(round: &str) -> (u32, u32, u32) {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let round_splits: Vec<&str> = round.split(", ").collect();

    for round_split in round_splits {
        let split_values: Vec<&str> = round_split.split(' ').collect();

        for (key, value) in split_values.iter().step_by(2).enumerate() {
            if let Ok(num) = value.parse::<u32>() {
                match split_values[key + 1] {
                    "red" => red += num,
                    "green" => green += num,
                    "blue" => blue += num,
                    _ => (),
                }
            }
        }
    }

    return (red, green, blue);
}

fn get_game_id(line: &str) -> (u32, u32) {
    let space_option: Option<usize> = line.find(' ');
    let colon_option: Option<usize> = line.find(':');

    let space_index = match space_option {
        Some(idx) => idx,
        None => 0,
    };

    let colon_index = match colon_option {
        Some(idx) => idx,
        None => 0,
    };

    return (
        line[space_index + 1..colon_index]
            .parse::<u32>()
            .unwrap_or(0),
        colon_index as u32,
    );
}
