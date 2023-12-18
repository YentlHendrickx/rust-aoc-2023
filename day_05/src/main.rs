use std::collections::BTreeMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let lowest = solve(&input, false);
    println!("Lowest value: {}", lowest);

    let lowest_range = solve(&input, true);
    println!("Lowest range: {}", lowest_range);
}

fn solve(input: &str, part_two: bool) -> u64 {
    // Get first line of input
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let seeds = &first_line.split(" ").collect::<Vec<&str>>()[1..];

    // Now we get all of the maps
    let mut temp_map = BTreeMap::new();
    let mut map_key: String = String::new();
    let mut index: u32 = 0;
    for line in lines {
        let map = line.split(" ").collect::<Vec<&str>>();
        if map.len() == 1 {
            continue;
        }

        // Check if the first element is not a digit to identify a new map
        if map[0].parse::<u32>().is_err() {
            map_key = format!("{}_{}", index, map[0]);
            index += 1;
            temp_map.entry(map_key.clone()).or_insert_with(Vec::new);
        } else if let Some(vec) = temp_map.get_mut(&map_key) {
            vec.extend_from_slice(&map);
        } else {
            // Handle error: No corresponding map for these values
            eprintln!("Error: No map found for key '{}'", map_key);
        }
    }

    let mut final_map = BTreeMap::<&str, Vec<Vec<&str>>>::new();

    // Loop over keys of map and display
    for (key, value) in temp_map.iter() {
        // Slice up values into paris of 3
        let mut new_value = Vec::<Vec<&str>>::new();
        let mut temp = Vec::<&str>::new();
        for (i, val) in value.iter().enumerate() {
            temp.push(val);
            if (i + 1) % 3 == 0 {
                new_value.push(temp);
                temp = Vec::<&str>::new();
            }
        }

        // Store new vlaues into map
        final_map.insert(key, new_value);
    }

    let mut lowest_number = u64::MAX;

    // Loop over seeds
    // BRUTE FORCE FOR PART 2 BECAUSE I'M LAZY
    let start_index = 0;

    let end_index = seeds.len() - 2;
    let mut index: u32 = 0;

    for i in start_index..end_index {
        if index > (seeds.len() - 2) as u32 {
            break;
        }

        let seeds_list = if part_two {
            process_seeds(&seeds, index)
        } else {
            seeds.iter().map(|&s| s.to_string()).collect()
        };

        index += 2;

        // Loop over seeds
        for seed in seeds_list.iter() {
            // println!("Seed: {}", seed);

            // Loop over maps
            let mut current_value = seed.parse::<u64>().unwrap();
            for (key, value) in final_map.iter() {
                // println!("Key: {}", key);

                for val in value.iter() {
                    // The range is specified by the finaly value
                    let dest_lower = val[0].parse::<u64>().unwrap();
                    let source_lower = val[1].parse::<u64>().unwrap();
                    let range_size = val[2].parse::<u64>().unwrap();

                    let source_upper = source_lower + range_size - 1;

                    if current_value >= source_lower && current_value <= source_upper {
                        // Seed is within the source range, update current_value
                        let offset = current_value - source_lower;
                        // /*   println!(
                        //     "Seed value: {}, source_lower: {}, offset: {}",
                        //     current_value, source_lower, offset
                        // ); */
                        current_value = dest_lower + offset;
                        // println!("Current value: {}", current_value);
                        break; // Exit the loop as we found a match
                    }
                }
            }
            if current_value < lowest_number {
                lowest_number = current_value;
            }
        }
    }
    return lowest_number;
}

fn process_seeds(seeds: &[&str], index: u32) -> Vec<String> {
    let mut new_seeds_vec = Vec::new();
    let mut start_val: u64 = 0;

    let mut count = 1;

    for seed in index..(index + 2) {
        if count == 1 {
            start_val = seeds[seed as usize].parse::<u64>().unwrap();
            count += 1;
        } else {
            let range_val = start_val + seeds[seed as usize].parse::<u64>().unwrap();

            for i in start_val..=range_val {
                new_seeds_vec.push(i.to_string());
            }

            count = 1;
        }
    }

    return new_seeds_vec;
}
