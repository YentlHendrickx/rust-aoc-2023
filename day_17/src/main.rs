use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[derive(Eq, PartialEq, Debug, Hash, Ord, PartialOrd, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Coordinate {
    r: usize,
    c: usize,
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Vertex {
    pos: Coordinate,
    dir: Direction,
    cost: u32,
    steps: u32,
}

// Crucible sorting implementations
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Vertex {
    fn neighbors(&self, map: &Vec<Vec<usize>>, min: u32, max: u32) -> Vec<Self> {
        let rows = map.len();
        let cols = map[0].len();

        let mut neighbors = Vec::new();
        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            // Min needs to be enforced
            if self.dir != dir && self.steps < min {
                continue;
            }

            if self.dir == dir && self.steps == max {
                continue;
            }

            // Prevent backtracking
            if dir == self.dir.opposite_direction() {
                continue;
            }

            if let Some(pos) = self.pos.next(dir, rows, cols) {
                let cost = self.cost + map[pos.r][pos.c] as u32;
                let steps = if self.dir == dir { self.steps + 1 } else { 1 };

                neighbors.push(Vertex {
                    pos,
                    dir,
                    cost,
                    steps,
                });
            }
        }

        return neighbors;
    }
}

impl Coordinate {
    fn next(&self, dir: Direction, rows: usize, cols: usize) -> Option<Self> {
        let coordinate = match dir {
            Direction::North if self.r > 0 => Coordinate {
                r: self.r - 1,
                c: self.c,
            },
            Direction::East if self.c < (cols - 1) => Coordinate {
                r: self.r,
                c: self.c + 1,
            },
            Direction::South if self.r < (rows - 1) => Coordinate {
                r: self.r + 1,
                c: self.c,
            },
            Direction::West if self.c > 0 => Coordinate {
                r: self.r,
                c: self.c - 1,
            },

            _ => return None,
        };

        return Some(coordinate);
    }
}

impl Direction {
    fn opposite_direction(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./data/test2.txt").unwrap();
    let grid = parse(&input);

    let (part_1, part_2) = solve(grid.clone());
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn solve(map: Vec<Vec<usize>>) -> (u32, u32) {
    let part_1 = dijkstra(map.clone(), 0, 3);
    let part_2 = dijkstra(map.clone(), 4, 10);

    return (part_1, part_2);
}

fn dijkstra(map: Vec<Vec<usize>>, min: u32, max: u32) -> u32 {
    let mut min_heat = 0;

    let goal = Coordinate {
        r: map.len() - 1,
        c: map[0].len() - 1,
    };

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let start_vertices = vec![
        // We skip first since that doesn't count towards total
        Vertex {
            pos: Coordinate { r: 0, c: 1 },
            dir: Direction::East,
            cost: map[0][1] as u32,
            steps: 1,
        },
        Vertex {
            pos: Coordinate { r: 1, c: 0 },
            dir: Direction::South,
            cost: map[1][0] as u32,
            steps: 1,
        },
    ];

    for start in start_vertices {
        heap.push(start);
    }

    while let Some(vertex) = heap.pop() {
        if vertex.pos == goal && vertex.steps >= min {
            min_heat = vertex.cost;
            break;
        }

        for neighbor in vertex.neighbors(&map, min, max) {
            if seen.insert((neighbor.pos, neighbor.dir, neighbor.steps)) {
                heap.push(neighbor);
            }
        }
    }

    return min_heat;
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    return input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
}
