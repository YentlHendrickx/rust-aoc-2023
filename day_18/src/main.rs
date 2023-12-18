use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct DigNode {
    direction: Direction,
    distance: u32,
    color: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let dig_plan = parse(&input);

    let coordinates = make_coordinates(&dig_plan);
    let part_1 = solve(&coordinates);
    println!("Part 1: {}", part_1);

    let new_dig_plan = convert_dig_plan(&dig_plan);
    let coordinates = make_coordinates(&new_dig_plan);
    let part_2 = solve(&coordinates);
    println!("Part 2: {}", part_2);
}

fn solve(coordinates: &Vec<Coordinate>) -> i64 {
    let area = shoelace(&coordinates);
    let result = area + (coordinates.len() as i64 / 2).abs() + 1;
    return result;
}

fn shoelace(coordinates: &Vec<Coordinate>) -> i64 {
    let mut s1 = 0;
    let mut s2 = 0;
    for window in coordinates.windows(2) {
        let c1 = &window[0];
        let c2 = &window[1];

        s1 += c1.x * c2.y;
        s2 += c1.y * c2.x;
    }

    let area = (s1 - s2).abs() / 2;

    return area;
}

fn make_coordinates(dig_plan: &Vec<DigNode>) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = Vec::new();

    let default_coordinate = Coordinate { x: 0, y: 0 };
    for node in dig_plan {
        match node.direction {
            Direction::Left => {
                for _ in 0..node.distance {
                    coordinates.push(Coordinate {
                        x: coordinates.last().unwrap_or(&default_coordinate).x - 1,
                        y: coordinates.last().unwrap_or(&default_coordinate).y,
                    });
                }
            }
            Direction::Right => {
                for _ in 0..node.distance {
                    coordinates.push(Coordinate {
                        x: coordinates.last().unwrap_or(&default_coordinate).x + 1,
                        y: coordinates.last().unwrap_or(&default_coordinate).y,
                    });
                }
            }
            Direction::Up => {
                for _ in 0..node.distance {
                    coordinates.push(Coordinate {
                        x: coordinates.last().unwrap_or(&default_coordinate).x,
                        y: coordinates.last().unwrap_or(&default_coordinate).y - 1,
                    });
                }
            }
            Direction::Down => {
                for _ in 0..node.distance {
                    coordinates.push(Coordinate {
                        x: coordinates.last().unwrap_or(&default_coordinate).x,
                        y: coordinates.last().unwrap_or(&default_coordinate).y + 1,
                    });
                }
            }
        }
    }

    return coordinates;
}

fn convert_dig_plan(dig_plan: &Vec<DigNode>) -> Vec<DigNode> {
    let mut new_dig_plan = Vec::new();

    for node in dig_plan {
        let color = node.color.clone();
        let distance = hex_to_u32(&color[0..5]);
        let direction = match &color[5..6] {
            "2" => Direction::Left,
            "0" => Direction::Right,
            "3" => Direction::Up,
            "1" => Direction::Down,
            _ => panic!("Invalid direction"),
        };

        let new_node = DigNode {
            direction: direction,
            distance,
            color: node.color.clone(),
        };

        new_dig_plan.push(new_node);
    }

    return new_dig_plan;
}

fn parse(input: &str) -> Vec<DigNode> {
    let mut result = Vec::new();

    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        let direction = match split[0] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction"),
        };

        let distance = split[1].parse::<u32>().unwrap();

        let color = split[2].replace("(", "").replace(")", "").replace("#", "");

        result.push(DigNode {
            direction,
            distance,
            color: color.to_string(),
        })
    }

    return result;
}

fn hex_to_u32(hex: &str) -> u32 {
    return u32::from_str_radix(hex, 16).unwrap();
}
