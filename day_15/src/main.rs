use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Instruction {
    lens: Lens,
    location: u32,
    operator: char,
    full_hash: u32,
}

#[derive(Debug, Default, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Lens {
    label: String,
    fs: u32,
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let parsed = parse_input(&input);
    let result = part_1(&parsed);
    println!("part 1: {}", result);

    let result = part_2(&parsed);
    println!("part 2: {}", result);
}

fn part_1(input: &Vec<Instruction>) -> u32 {
    let mut result: u32 = 0;

    for instruction in input {
        result += instruction.full_hash;
    }

    return result;
}

fn part_2(input: &Vec<Instruction>) -> u32 {
    let mut result = 0;

    let box_map = perform_instruction(&input);

    for (i, (_, box_contents)) in box_map.iter().sorted().enumerate() {
        let mut focal_power = 0;
        for (j, lens) in box_contents.iter().enumerate() {
            focal_power += (i as u32 + 1) * lens.fs * (j as u32 + 1);
        }

        result += focal_power;
    }

    return result;
}

fn perform_instruction(input: &Vec<Instruction>) -> HashMap<u32, Vec<Lens>> {
    let mut box_map = HashMap::<u32, Vec<Lens>>::new();
    for i in 0..256 {
        box_map.insert(i, Vec::new());
    }

    for instruction in input {
        let current_box = box_map.get_mut(&instruction.location).unwrap();
        let mut box_copy: Vec<Lens> = Vec::new();

        if instruction.operator == '-' {
            for lens in current_box {
                if lens.label == instruction.lens.label {
                    continue;
                }

                box_copy.push(lens.clone());
            }

            box_map.insert(instruction.location, box_copy);
        } else if instruction.operator == '=' {
            let mut added = false;
            for lens in current_box {
                if lens.label == instruction.lens.label {
                    box_copy.push(instruction.lens.clone());
                    added = true;
                } else {
                    box_copy.push(lens.clone());
                }
            }

            if !added {
                box_copy.push(instruction.lens.clone());
            }

            box_map.insert(instruction.location, box_copy);
        }
    }

    return box_map;
}

fn make_hash(input: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;

    for c in input {
        result += *c as u32;
        result *= 17;
        result = result % 256;
    }

    return result;
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut parsed: Vec<Instruction> = Vec::new();
    let split = input.split(",");

    for s in split {
        let split_instruction = s.split_at(s.find("-").unwrap_or(s.find("=").unwrap_or(0)));

        let label = split_instruction.0;
        let location = make_hash(&label.as_bytes().to_vec());
        let operator = split_instruction.1.chars().nth(0).unwrap();

        let mut focal_length: i32 = 0;
        if operator == '=' {
            focal_length = split_instruction.1.split_at(1).1.parse::<i32>().unwrap();
        }

        let instruction: Instruction = Instruction {
            lens: Lens {
                label: String::from(label),
                fs: focal_length as u32,
            },
            location: location,
            operator: operator,
            full_hash: make_hash(&s.as_bytes().to_vec()),
        };

        parsed.push(instruction);
    }

    return parsed;
}
