use std::collections::HashSet;
use std::fs;

#[non_exhaustive]
struct Directions;

impl Directions {
    pub const UP: (i32, i32) = (0, 1);
    pub const DOWN: (i32, i32) = (0, -1);
    pub const LEFT: (i32, i32) = (-1, 0);
    pub const RIGHT: (i32, i32) = (1, 0);
    pub const ALL: [(i32, i32); 4] = [
        Directions::UP,
        Directions::DOWN,
        Directions::LEFT,
        Directions::RIGHT,
    ];
}

fn main() {
    let input = fs::read_to_string("./data/test.txt").unwrap();
    // let parsed = parse(&input);
    //

    // General outline
    // -> recreate the grid but only keep track of the vertices (forks)
    // -> BFS to get distances between all vertices
    // -> DFS to find the longest path, if part 1 follow the >, <, ^, v rules
}

fn solve(grid: &Vec<Vec<char>>) -> u64 {
    // First find all the forks (vertices)
    let mut vertices: HashSet<(u32, u32)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Count neighbors that are not walls
            let mut neighbors = 0;
            for (dx, dy) in Directions::ALL.iter() {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < grid[y].len() as i32 && ny >= 0 && ny < grid.len() as i32 {
                    if grid[ny as usize][nx as usize] != '#' {
                        neighbors += 1;
                    }
                }
            }

            // If we have more than 2 neighbors, we have a vertex
            if neighbors > 2 {
                vertices.insert((x as u32, y as u32));
            }
        }
    }
    // Store the start position and end positino in the vertices
    // Start is always at (1,0) and end is always at (len-2,len-1)
    vertices.insert((1, 0));
    vertices.insert((grid[0].len() as u32 - 2, grid.len() as u32 - 1));

    return 0;
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut coordinates: Vec<Vec<char>> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, typ) in line.chars().enumerate() {
            coordinates[row][col] = typ;
        }
    }
    return coordinates;
}
