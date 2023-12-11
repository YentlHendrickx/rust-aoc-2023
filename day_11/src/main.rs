use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let map = get_expanded_map(&input, false);
    let part_1 = solve(&map);
    println!("Part 1: {}", part_1);

    let map = get_expanded_map(&input, true);
    let part_2 = solve(&map);
    println!("Part 2: {}", part_2);
}

fn solve(map: &HashMap<(u64, u64), u64>) -> i64 {
    let mut result: i64 = 0;

    // Loop over all values of the map
    for (key, value) in map.iter() {
        // Loop again
        for (key_2, value_2) in map.iter() {
            if key == key_2 || value < value_2 {
                continue;
            }

            // Calculate the distance
            let distance =
                (key.0 as i64 - key_2.0 as i64).abs() + (key.1 as i64 - key_2.1 as i64).abs();

            result += distance as i64;
        }
    }

    return result;
}

fn get_expanded_map(input: &str, part_two: bool) -> HashMap<(u64, u64), u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid: HashMap<(u64, u64), u64> = HashMap::new();

    // Identify empty rows and columns
    let mut empty_rows = vec![true; lines.len()];
    let mut empty_cols = vec![true; lines[0].len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                empty_rows[y] = false;
                empty_cols[x] = false;
            }
        }
    }

    let expansion_constant = 1000000 - 1;
    let empty_col_add = if part_two { expansion_constant } else { 1 };
    let empty_row_add = if part_two { expansion_constant } else { 1 };

    // Expand grid and return map
    let mut galaxy_start = 1;
    let mut new_y = 0;
    for (y, line) in lines.iter().enumerate() {
        if empty_rows[y] {
            new_y += empty_row_add;
        }

        let mut new_x = 0;
        for (x, char) in line.chars().enumerate() {
            if empty_cols[x] {
                new_x += empty_col_add;
            }

            if char == '#' {
                grid.insert((new_y as u64, new_x as u64), galaxy_start);
                galaxy_start += 1;
            }

            new_x += 1;
        }

        new_y += 1;
    }

    return grid;
}
