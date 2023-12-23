use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let modules = parse(&input);

    // After looking at the data, it seems like the 'program' is 4 binary counters
    // For part 2 we need to find the number of cycles to get to 'rx'
    // Thus we need to find the LCM of the 4 counters
    let (part_1, part_2) = solve(&modules);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

// Copied from day_08
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
}

fn solve(modules: &HashMap<String, (Vec<String>, bool)>) -> (u64, u64) {
    let mut modules = modules.clone();

    let mut queue: LinkedList<(String, String, String)> = LinkedList::new();

    let total_cycles = 1000;
    let mut cycles = 0;

    let mut lows = 0;
    let mut highs = 0;

    let mut part_one_result: u64 = 0;

    let mut binary_counters: HashMap<String, u32> = HashMap::new();
    // Find the end of the binary counters
    for (key, value) in modules.iter() {
        // Find the destination in the hashmap (rx)
        for dest in value.clone().0 {
            if dest == "rx" {
                // Now we find all the destinations of this key
                for (_k, v) in modules.iter() {
                    if v.0.contains(&key) {
                        // Insert into hashmap
                        binary_counters.insert(_k.to_string(), 0);
                    }
                }
            }
        }
    }

    loop {
        queue.push_back((
            "broadcaster".to_string(),
            "button".to_string(),
            "LOW".to_string(),
        ));

        while queue.len() > 0 {
            let (source, _from, sig) = queue.pop_front().unwrap();

            if sig == "LOW" {
                lows += 1;
            } else {
                highs += 1;
            }

            if sig == "LOW" && binary_counters.contains_key(&source) {
                // If the counter is at 0 set it to current cycle
                if binary_counters.get(&source).unwrap() == &0 {
                    // + 1 because we start at 0
                    binary_counters.insert(source.clone(), cycles + 1);
                }

                // If we have all the counters, calculate the LCM
                if binary_counters.values().all(|&x| x != 0) {
                    let mut result_2 = 1;
                    for (_k, v) in binary_counters.iter() {
                        result_2 = lcm(result_2, *v as u64);
                    }

                    return (part_one_result, result_2);
                }
            }

            // If it's a broadcaster add all the destinations to the queue
            if source == "broadcaster" {
                // Find broadcaster in the hashmap
                let destinations = modules.get(&source).unwrap();
                for dest in destinations.0.iter() {
                    queue.push_back((dest.to_string(), source.clone(), sig.clone()));
                }
            } else if source.chars().nth(0) == Some('%') {
                // Flip flop
                if sig == "HIGH" {
                    continue;
                }

                // Find source in the hashmap
                let module = modules.get(&source).unwrap();
                // Check the current state
                let new_state = !module.1;

                // For all destinations of the source add them to the queue
                for dest in module.clone().0.iter() {
                    queue.push_back((
                        dest.to_string(),
                        source.clone(),
                        if new_state {
                            "HIGH".to_string()
                        } else {
                            "LOW".to_string()
                        },
                    ));
                }

                modules.insert(source.clone(), (module.0.clone(), new_state));
            } else if source.chars().nth(0) == Some('&') {
                // Find the module
                let module = modules.get(&source).unwrap();
                let destinations = module.clone().0;
                // Loop over all destinations
                let mut new_state = false;

                for (_module, info) in modules.clone().iter() {
                    if !info.0.contains(&source) {
                        continue;
                    }

                    if info.1 == false {
                        new_state = true;
                    }
                }

                for dest in destinations.iter() {
                    queue.push_back((
                        dest.to_string(),
                        source.clone(),
                        if new_state {
                            "HIGH".to_string()
                        } else {
                            "LOW".to_string()
                        },
                    ));
                }

                modules.insert(source.clone(), (module.0.clone(), new_state));
            } else {
                continue;
            }
        }

        cycles += 1;

        if total_cycles == cycles {
            part_one_result = lows * highs;
        }
    }
}

fn parse(input: &str) -> HashMap<String, (Vec<String>, bool)> {
    let mut modules: HashMap<String, (Vec<String>, bool)> = HashMap::new();

    for line in input.lines() {
        let parts = line
            .split(" -> ")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let source = &parts[0];
        let destinations = &parts[1];

        let destinations = destinations
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        modules.insert(source.to_string(), (destinations, false));
    }

    // For ease of use we modify all the destinations to add the type in front
    for (_key, module) in modules.clone().iter_mut() {
        let mut new_destinations: Vec<String> = Vec::new();
        for dest in module.0.iter() {
            // Find the destination in the hashmap (by appending '%' or '&' ad the start)
            let keys = vec![format!("%{}", dest), format!("&{}", dest)];
            let mut added = false;
            for key in keys.iter() {
                if let Some(_m) = modules.get(key) {
                    // Add the type to the destination
                    new_destinations.push(key.to_string());
                    added = true;
                    break;
                }
            }

            if !added {
                new_destinations.push(dest.to_string());
            }
        }

        modules.insert(_key.to_string(), (new_destinations, false));
    }

    return modules;
}
