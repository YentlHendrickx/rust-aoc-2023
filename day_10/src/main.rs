use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let grid = parse_input(&input);
    let part_1 = solve_1(&grid);
    println!("Part 1: {}", part_1.len() / 2);

    let part_2 = solve_2(&grid);
    println!("Part 2: {}", part_2)
}

fn get_start(grid: &Grid) -> [(i32, i32); 2] {
    let start_pos = grid.start_pos;
    let mut start_directions = Vec::new();

    for (x, y) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let pipe_pos = (start_pos.0 + x, start_pos.1 + y);

        if let Some(pipe) = grid.pipes.get(&pipe_pos) {
            for direction in pipe.iter() {
                if (pipe_pos.0 + direction.0, pipe_pos.1 + direction.1) == start_pos {
                    start_directions.push((-direction.0, -direction.1));
                    break;
                }
            }

            if start_directions.len() == 2 {
                break;
            }
        }
    }

    return start_directions.try_into().unwrap();
}

fn solve_1(grid: &Grid) -> HashSet<(i32, i32)> {
    let mut current_pos = grid.start_pos;
    let mut main_loop = HashSet::from([current_pos]);
    let starting_position = get_start(grid);

    'main_loop: loop {
        if let Some(pipe) = grid.pipes.get(&current_pos).or(Some(&starting_position)) {
            for direction in pipe {
                let adjacent_pipe_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);

                if !main_loop.contains(&adjacent_pipe_pos) {
                    current_pos = adjacent_pipe_pos;
                    main_loop.insert(current_pos);
                    continue 'main_loop;
                }
            }

            return main_loop;
        }
    }
}

fn solve_2(grid: &Grid) -> usize {
    let main = solve_1(grid);
    let starting_pipe = get_start(grid);
    let mut inside_count = 0;

    for y in 0..=grid.bottom_right.1 {
        let mut outside = true;

        for x in 0..=grid.bottom_right.0 {
            if let Some(position) = main.get(&(x, y)) {
                let directions = grid.pipes.get(position).unwrap_or(&starting_pipe);

                if directions.contains(&(0, 1)) {
                    outside = !outside;
                }
            } else if !outside {
                inside_count += 1;
            }
        }
    }

    return inside_count;
}

struct Grid {
    start_pos: (i32, i32),
    pipes: HashMap<(i32, i32), [(i32, i32); 2]>,
    bottom_right: (i32, i32),
}

fn parse_input(input: &str) -> Grid {
    let mut grid = Grid {
        start_pos: (0, 0),
        pipes: HashMap::new(),
        bottom_right: (
            input.lines().next().unwrap().chars().count() as i32 - 1,
            input.lines().count() as i32 - 1,
        ),
    };

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'S' => grid.start_pos = (x as i32, y as i32),
            '.' => (),
            _ => {
                grid.pipes.insert(
                    (x as i32, y as i32),
                    match c {
                        '|' => [(0, -1), (0, 1)],
                        '-' => [(-1, 0), (1, 0)],
                        'L' => [(0, -1), (1, 0)],
                        'J' => [(0, -1), (-1, 0)],
                        '7' => [(-1, 0), (0, 1)],
                        'F' => [(1, 0), (0, 1)],
                        _ => unreachable!(),
                    },
                );
            }
        });
    });

    return grid;
}
