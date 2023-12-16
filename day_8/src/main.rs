use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let part_1 = solve_1(&input);
    println!("Part 1: {}", part_1);

    let part_2 = solve_2(&input);
    println!("Part 2: {}", part_2)
}

fn solve_1(input: &str) -> u64 {
    let mut result: u64 = 0;
    let left_right = get_left_right(input);
    // println!("{:?}", left_right);

    let map = get_map(input);
    let mut current_element: String = "AAA".to_string();
    let target_element = "ZZZ";
    let mut current_instruction = 0;

    while current_element != target_element {
        result += 1;
        let next_element = map.get(&current_element).unwrap();
        // println!("{:?}", next_element);

        let current_pick = left_right[current_instruction];
        if current_pick == 'L' {
            current_element = next_element[0].clone();
        } else {
            current_element = next_element[1].clone();
        }

        current_instruction += 1;
        if current_instruction == left_right.len() {
            current_instruction = 0;
        }
    }

    return result;
}

fn solve_2(input: &str) -> u64 {
    let left_right = get_left_right(input);
    let map = get_map(input);
    let mut current_instruction = 0;

    let mut end_elements = Vec::new();
    let mut start_elements = Vec::new();

    for (key, _value) in map.iter() {
        if key.ends_with("A") {
            start_elements.push(key);
        } else if key.ends_with("Z") {
            end_elements.push(key);
        }
    }

    let mut all_steps: Vec<u64> = Vec::new();

    for current_element in start_elements {
        let mut current_element = current_element.clone();
        let mut steps = 0;
        while !end_elements.contains(&&current_element) {
            steps += 1;
            let next_element = map.get(&current_element).unwrap();
            // println!("{:?}", next_element);

            let current_pick = left_right[current_instruction];
            if current_pick == 'L' {
                current_element = next_element[0].clone();
            } else {
                current_element = next_element[1].clone();
            }

            current_instruction += 1;
            if current_instruction == left_right.len() {
                current_instruction = 0;
            }
        }

        all_steps.push(steps);
    }

    let mut current_lcm = all_steps[0];
    for i in 1..all_steps.len() {
        current_lcm = lcm(current_lcm, all_steps[i]);
    }

/*     println!(
        "{:?}, we have a Lowest Common Multiple (LCM) of {}"
        all_steps, current_lcm
    ) */;

    return current_lcm;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
}

fn get_left_right(input: &str) -> Vec<char> {
    return input.lines().nth(0).unwrap().chars().collect();
}

fn get_map(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for lines in input.lines().skip(2).into_iter() {
        let split: Vec<&str> = lines.split(" ").collect();

        let left_side = split[0].to_string();
        let first_right = split[2].trim().replace(",", "").replace("(", "");
        let second_right = split[3].trim().replace(")", "");

        // println!("{} -> {}, {}", left_side, first_right, second_right);

        map.insert(left_side, vec![first_right, second_right]);
    }

    return map;
}
