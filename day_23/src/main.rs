use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};
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
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let parsed = parse(&input);

    // General outline
    // -> recreate the grid but only keep track of the vertices (forks)
    // -> BFS to get distances between all vertices
    // -> DFS to find the longest path, if part 1 follow the >, <, ^, v rules
    let part_1 = solve(&parsed, false);
    println!("Part 1: {}", part_1);

    let part_2 = solve(&parsed, true);
    println!("Part 2: {}", part_2);
}

fn solve(grid: &Vec<Vec<char>>, part_two: bool) -> u64 {
    // First find all the forks (vertices)
    let mut vertices: BTreeSet<(u32, u32)> = BTreeSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Count neighbors that are not walls
            let mut neighbors = 0;
            for (dx, dy) in Directions::ALL.iter() {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < grid[y].len() as i32 && ny >= 0 && ny < grid.len() as i32 {
                    if grid[nx as usize][ny as usize] != '#' {
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

    // Next we need to find the distances between all vertices using BFS
    let mut dist_map: HashMap<(u32, u32), Vec<((u32, u32), u64)>> = HashMap::new();

    // BFS
    // Loop over vertices 2 at a time
    for (x_p, y_p) in vertices.iter() {
        let mut queue: LinkedList<((u32, u32), u64)> = LinkedList::new();

        // Visited map
        let mut visited: HashMap<(u32, u32), bool> = HashMap::new();

        // Add the start position to the queue
        queue.push_back(((*x_p, *y_p), 0));

        while !queue.is_empty() {
            let ((x, y), dist) = queue.pop_front().unwrap();

            // Check if we have already visited this node
            if visited.contains_key(&(x, y)) {
                continue;
            }

            // Mark as visited
            visited.insert((x, y), true);

            // Check if we are at a vertex
            if vertices.contains(&(x, y)) && (x, y) != (*x_p, *y_p) {
                // Add to the distance map
                if dist_map.contains_key(&(*x_p, *y_p)) {
                    dist_map
                        .get_mut(&(*x_p, *y_p))
                        .unwrap()
                        .push(((x, y), dist));
                } else {
                    dist_map.insert((*x_p, *y_p), vec![((x, y), dist)]);
                }
                continue;
            }

            // Add all neighbors to the queue
            for (dx, dy) in Directions::ALL.iter() {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0
                    && nx < grid.len() as i32
                    && ny >= 0
                    && ny < grid[y as usize].len() as i32
                {
                    if grid[nx as usize][ny as usize] != '#' {
                        if !part_two {
                            // Make sure that if we are walking into a ^, v, <, > that we are not allowing to walk through it
                            if grid[x as usize][y as usize] == '^' && *dy != -1 {
                                continue;
                            } else if grid[x as usize][y as usize] == 'v' && *dy != 1 {
                                continue;
                            } else if grid[x as usize][y as usize] == '<' && *dx != -1 {
                                continue;
                            } else if grid[x as usize][y as usize] == '>' && *dx != 1 {
                                continue;
                            }
                        }

                        queue.push_back(((nx as u32, ny as u32), dist + 1));
                    }
                }
            }
        }
    }

    // DFS TIME!

    let mut queue: LinkedList<((u32, u32), u64, HashSet<(u32, u32)>)> = LinkedList::new();
    let visited: HashSet<(u32, u32)> = HashSet::new();

    // Add all postions to the queue that are connected to the start
    // Get start position from dist map
    for (vert_2, dist) in dist_map.get(&(1, 0)).unwrap() {
        queue.push_back((*vert_2, *dist, visited.clone()));
    }

    let result = dfs(
        &mut queue,
        &dist_map,
        (grid[0].len() as u32 - 2, grid.len() as u32 - 1),
    );

    return result;
}

fn dfs(
    queue: &mut LinkedList<((u32, u32), u64, HashSet<(u32, u32)>)>,
    dist_map: &HashMap<(u32, u32), Vec<((u32, u32), u64)>>,
    // visited: &mut HashSet<((u32, u32), (u32, u32))>,
    end_location: (u32, u32),
) -> u64 {
    let mut result = 0;
    while !queue.is_empty() {
        let (vert, dist, visited) = queue.pop_front().unwrap();
        let mut visited = visited.clone();

        if dist > result && vert == end_location {
            result = dist;
        }

        // Check if we have already visited this node
        if visited.contains(&vert) {
            continue;
        }

        // Mark as visited
        visited.insert(vert);

        // Find all the next vertices
        for (next_vert, next_dist) in dist_map.get(&vert).unwrap_or(&vec![((0, 0), 0)]).iter() {
            if next_vert == &(0, 0) {
                break;
            }

            if visited.contains(&(*next_vert)) {
                continue;
            }
            queue.push_back((*next_vert, dist + next_dist, visited.clone()));
        }

        visited.remove(&vert);
    }

    return result;
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut coordinates: Vec<Vec<char>> = Vec::new();

    // Create all coordinates
    for _ in 0..input.lines().count() {
        coordinates.push(vec![' '; input.lines().count()]);
    }

    for (row, line) in input.lines().enumerate() {
        for (col, typ) in line.chars().enumerate() {
            coordinates[col][row] = typ;
        }
    }
    return coordinates;
}
