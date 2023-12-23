use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;

#[non_exhaustive]
struct Direction;

impl Direction {
    pub const UP: (i32, i32) = (0, -1);
    pub const DOWN: (i32, i32) = (0, 1);
    pub const RIGHT: (i32, i32) = (1, 0);
    pub const LEFT: (i32, i32) = (-1, 0);
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let parsed = parse(&input);

    let (part_1, part_2) = solve(parsed);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn solve(grid: Vec<Vec<char>>) -> (u64, usize) {
    let mut visited_tile: HashMap<(usize, usize), i32> = HashMap::new();
    let mut start: (usize, usize) = (0, 0);
    // Intialize the hashmap with all tiles and false
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // Only insert valid locations
            if grid[i][j] != '#' {
                visited_tile.insert((i, j), -1);
            }
            if grid[i][j] == 'S' {
                start = (i, j);
            }
        }
    }

    let mut step_queue: LinkedList<(usize, usize, u64)> = LinkedList::new();

    // Add initial positions to queue
    let positions = get_positions(start, &grid);

    for position in positions {
        step_queue.push_back((position.0, position.1, 1));
    }

    while step_queue.len() > 0 {
        // Get the next queue position
        let current_position = step_queue.pop_front().unwrap();

        if visited_tile
            .get(&(current_position.0, current_position.1))
            .unwrap()
            == &-1
        {
            visited_tile.insert(
                (current_position.0, current_position.1),
                current_position.2 as i32,
            );
        } else {
            continue;
        }

        // Use this position to analyze the next positions
        let positions = get_positions((current_position.1, current_position.0), &grid);

        // Add the positions to the queue
        for position in positions {
            step_queue.push_back((position.1, position.0, current_position.2 + 1));
        }
    }

    // count how many values in the hashmap are 64 or even and smaller than 64
    let part_1 = visited_tile
        .values()
        .filter(|v| **v % 2 == 0 && **v <= 64)
        .count();

    // These maths are complicated...
    let even_corners = visited_tile
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited_tile
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited_tile.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited_tile.values().filter(|v| **v % 2 == 1).count();

    // We get this based on grid size
    let n = 202300;

    let part_2 = ((n + 1) * (n + 1)) * odd_full + n * n * even_full - (n + 1) * odd_corners
        + n * even_corners;

    return (part_1 as u64, part_2);
}

// Analyze a position and return all posible locations
fn get_positions(current_position: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    // Check all 4 directions
    let mut directions: Vec<(i32, i32)> = Vec::new();
    directions.push(Direction::UP);
    directions.push(Direction::DOWN);
    directions.push(Direction::LEFT);
    directions.push(Direction::RIGHT);

    for direction in directions {
        let new_x = current_position.1 as i32 + direction.0;
        let new_y = current_position.0 as i32 + direction.1;
        // Check if the position is valid
        if new_x < 0 || new_y < 0 || new_x >= grid.len() as i32 || new_y >= grid.len() as i32 {
            break;
        }

        if grid[new_y as usize][new_x as usize] != '#' {
            result.push((new_x as usize, new_y as usize));
        }
    }

    return result;
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }

    return result;
}
