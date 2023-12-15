use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Clone)]
enum PlatformType {
    ROUND,
    CUBE,
    EMPTY,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let (parsed, (grid_x, grid_y)) = parse_input(&input);

    let slide = slide_dir(parsed, grid_x, grid_y, Direction::NORTH);

    let result = part_1(&slide, grid_x, grid_y);
    println!("part 1: {}", result);
    let result = part_2(slide, grid_x, grid_y);
    println!("part 2: {}", result);
}

fn part_1(map: &HashMap<(u32, u32), PlatformType>, grid_x: u32, grid_y: u32) -> u32 {
    let mut total = 0;
    for y in 0..grid_y {
        for x in 0..grid_x {
            let platform_type = map.get(&(x, y)).unwrap_or(&PlatformType::EMPTY);
            if platform_type == &PlatformType::ROUND {
                total += grid_y - y;
            }
        }
    }

    return total;
}

fn clone_map<K, V>(map: &HashMap<K, V>) -> HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
    V: std::clone::Clone,
{
    let mut output = HashMap::new();
    for (k, v) in map {
        output.insert(k.clone(), v.clone());
    }
    return output;
}

fn part_2(map: HashMap<(u32, u32), PlatformType>, grid_x: u32, grid_y: u32) -> u32 {
    let mut result = 0;

    let mut slide: HashMap<(u32, u32), PlatformType> = map;
    let cycles = 1000000000;
    let mut repeating_results: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut target = 0;

    while target < cycles {
        target += 1;
        // Every 1000 cycles we print the time and progress
        let mut current_dir = Direction::NORTH;
        result = part_1(&slide, grid_x, grid_y);
        for _j in 0..4 {
            slide = slide_dir(clone_map(&slide), grid_x, grid_y, current_dir.clone());

            current_dir = match current_dir {
                Direction::NORTH => Direction::EAST,
                Direction::EAST => Direction::SOUTH,
                Direction::SOUTH => Direction::WEST,
                Direction::WEST => Direction::NORTH,
            };
        }

        // Check if we have a repeating result
        repeating_results
            .entry(result)
            .and_modify(|v| v.push(target))
            .or_insert_with(|| vec![target]);

        let timestamps = repeating_results.get(&result).unwrap();

        if timestamps.len() >= 12 {
            let cycle_length = timestamps[timestamps.len() - 1] - timestamps[timestamps.len() - 2];
            if cycle_length == timestamps[timestamps.len() - 2] - timestamps[timestamps.len() - 3] {
                let amt = (cycles - target) / cycle_length;
                target += amt * cycle_length;
            }
        }
    }

    // print repeating results
    for (k, v) in repeating_results {
        println!("{}: {:?}", k, v);
    }

    return result;
}

fn _print_map(map: &HashMap<(u32, u32), PlatformType>, grid_x: u32, grid_y: u32) {
    for y in 0..grid_y {
        for x in 0..grid_x {
            let platform_type = map.get(&(x, y)).unwrap();
            let c = match platform_type {
                PlatformType::ROUND => 'O',
                PlatformType::CUBE => '#',
                PlatformType::EMPTY => '.',
            };
            print!("{}", c);
        }
        println!();
    }
    println!("-------------------");
}

fn slide_dir(
    map: HashMap<(u32, u32), PlatformType>,
    grid_x: u32,
    grid_y: u32,
    dir: Direction,
) -> HashMap<(u32, u32), PlatformType> {
    let mut output: HashMap<(u32, u32), PlatformType> = map;

    // Depending on the direction we need to loop differently
    // If we are going north we need to start at the bottom
    // If we are going east we need to start at the left
    // If we are going south we need to start at the top
    // If we are going west we need to start at the right
    let mut y = -1;
    let mut start_x = -1;
    let mut x_modifier: i32 = 1;
    let mut y_modifier: i32 = 1;

    match dir {
        Direction::NORTH => {
            y_modifier = 1;
            y = -1;
        }
        Direction::EAST => {
            x_modifier = -1;
            start_x = grid_x as i32;
        }
        Direction::SOUTH => {
            y_modifier = -1;
            y = grid_y as i32;
        }
        Direction::WEST => {
            x_modifier = 1;
            start_x = -1;
        }
    }

    for _i in 0..grid_y {
        y += y_modifier;
        let mut x = start_x;
        for _j in 0..grid_x {
            x += x_modifier;
            let current_key = (x as u32, y as u32);
            let current_platform = output.get(&current_key).unwrap_or(&PlatformType::EMPTY);

            if current_platform != &PlatformType::ROUND {
                continue;
            }

            if slide_break(dir.clone(), x as u32, y as u32, grid_x, grid_y) {
                continue;
            }

            let mut current_x = x as u32;
            let mut current_y = y as u32;
            loop {
                let next_key = match dir {
                    Direction::NORTH => (current_x, cmp::max(current_y as i32 - 1, 0) as u32),
                    Direction::EAST => (cmp::min(current_x + 1, grid_x as u32 - 1), current_y),
                    Direction::SOUTH => (current_x, cmp::min(current_y + 1, grid_y as u32 - 1)),
                    Direction::WEST => (cmp::max(current_x as i32 - 1, 0) as u32, current_y),
                };

                let next_platform = output.get(&next_key).unwrap_or(&PlatformType::EMPTY);

                if next_platform != &PlatformType::EMPTY {
                    break;
                }

                current_x = next_key.0;
                current_y = next_key.1;

                if slide_break(dir, current_x, current_y, grid_x, grid_y) {
                    break;
                }
            }

            // Check if we are at the same position
            if current_x == x as u32 && current_y == y as u32 {
                continue;
            }
            let new_key = (current_x, current_y);
            output.insert(new_key, PlatformType::ROUND);
            output.insert(current_key, PlatformType::EMPTY);
        }
    }

    return output;
}

fn slide_break(dir: Direction, x: u32, y: u32, grid_x: u32, grid_y: u32) -> bool {
    if dir == Direction::NORTH && y == 0 {
        return true;
    } else if dir == Direction::EAST && x == grid_x - 1 {
        return true;
    } else if dir == Direction::SOUTH && y == grid_y - 1 {
        return true;
    } else if dir == Direction::WEST && x == 0 {
        return true;
    }
    return false;
}

fn parse_input(input: &str) -> (HashMap<(u32, u32), PlatformType>, (u32, u32)) {
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let platform_type = match c {
                'O' => PlatformType::ROUND,
                '#' => PlatformType::CUBE,
                '.' => PlatformType::EMPTY,
                _ => panic!("Invalid input"),
            };
            result.insert((x as u32, y as u32), platform_type);
        }
    }

    let grid_size = (
        input.lines().next().unwrap().chars().count() as u32,
        input.lines().count() as u32,
    );

    return (result, grid_size);
}

// Write tests for part_1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./data/test.txt").unwrap();
        let (parsed, (grid_x, grid_y)) = parse_input(&input);
        let slide = slide_dir(parsed, grid_x, grid_y, Direction::NORTH);
        let result = part_1(&slide, grid_x, grid_y);
        assert_eq!(result, 136);
    }
}
