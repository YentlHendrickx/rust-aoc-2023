use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let parsed = parse_input(&input);
    let part_1 = solve(&parsed, (0, 0, 1));
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&parsed);
    println!("Part 2: {}", part_2);
}

fn part_2(grid: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    for (r, _row) in grid.iter().enumerate() {
        result = std::cmp::max(result, solve(&grid, (0, r as i32, 1)));
        result = std::cmp::max(
            result,
            solve(&grid, (grid[0].len() as i32 - 1, r as i32, 3)),
        );
    }

    for (c, _col) in grid[0].iter().enumerate() {
        result = std::cmp::max(result, solve(&grid, (c as i32, 0, 2)));
        result = std::cmp::max(result, solve(&grid, (c as i32, grid.len() as i32 - 1, 0)));
    }

    return result;
}

fn solve(grid: &Vec<Vec<char>>, start_pos: (i32, i32, i32)) -> u32 {
    let mut seen: Vec<(i32, i32)> = Vec::new();
    let mut seen_with_dir: Vec<(i32, i32, i32)> = Vec::new();

    let mut position: Vec<(i32, i32, i32)> = vec![start_pos];

    loop {
        let mut new_position: Vec<(i32, i32, i32)> = Vec::new();
        if position.len() == 0 {
            break;
        }

        for (x, y, d) in position.clone() {
            if x < 0 || y < 0 || x > grid.len() as i32 - 1 || y > grid[0].len() as i32 - 1 {
                continue;
            }

            if !seen.contains(&(x, y)) {
                seen.push((x, y));
            }

            if seen_with_dir.contains(&(x, y, d)) {
                continue;
            }

            seen_with_dir.push((x, y, d));

            let current_g = grid[y as usize][x as usize];
            if current_g == '.' {
                new_position.push(next_step((x, y, d)));
            } else if current_g == '/' {
                let new_dir = match d {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => 0,
                };

                new_position.push(next_step((x, y, new_dir)));
            } else if current_g == '\\' {
                let new_dir = match d {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => 0,
                };

                new_position.push(next_step((x, y, new_dir)));
            } else if current_g == '-' {
                // Left or right => continue
                if d == 1 || d == 3 {
                    new_position.push(next_step((x, y, d)));
                } else {
                    // Change dir left and right
                    new_position.push(next_step((x, y, 1)));
                    new_position.push(next_step((x, y, 3)));
                }
            } else if current_g == '|' {
                // Up or down => continue
                if d == 0 || d == 2 {
                    new_position.push(next_step((x, y, d)));
                } else {
                    // Change dir up and down
                    new_position.push(next_step((x, y, 0)));
                    new_position.push(next_step((x, y, 2)));
                }
            }
        }

        position = new_position.clone();
    }

    // Print the grid again but replace every seen with #
    /*     for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if seen.contains(&(x as i32, y as i32)) {
                print!("#",);
            } else {
                print!(".");
            }
        }
        println!();
    } */

    return seen.len() as u32;
}

fn next_step(current: (i32, i32, i32)) -> (i32, i32, i32) {
    let mut result = Vec::new();

    // Dir = 0 -> North
    // Dir = 1 -> East
    // Dir = 2 -> South
    // Dir = 3 -> West
    match current {
        (x, y, d) => {
            if d == 0 {
                result.push((x, y - 1, d));
            } else if d == 1 {
                result.push((x + 1, y, d));
            } else if d == 2 {
                result.push((x, y + 1, d));
            } else if d == 3 {
                result.push((x - 1, y, d));
            }
        }
    }

    return result[0];
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    return input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}
