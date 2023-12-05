#![allow(unused)]

use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn main() {
    solve_part_one();
    solve_part_two();
}

fn read_input_file() -> Vec<String> {
    let mut file = File::open("./data/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn solve_part_one() {
    let input = read_input_file();
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut seed_numbers = number_re
        .captures_iter(&input[0])
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for seed in seed_numbers.iter_mut() {
        let mut found = false;
        let mut i = 3;

        while i < input.len() {
            if !input[i].is_empty() && !found {
                let range = number_re
                    .captures_iter(&input[i])
                    .map(|cap| cap[1].parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                if *seed >= range[1] && *seed < range[1] + range[2] {
                    *seed = range[0] + (*seed - range[1]);
                    found = true;
                }
            } else if input[i].is_empty() {
                i += 1;
                found = false;
            }

            i += 1;
        }
    }

    println!("Result Part 1: {:?}", seed_numbers.iter().min().unwrap());
}

fn solve_part_two() {
    let mut input = read_input_file();
    input.push(String::new());
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut seed_numbers = number_re
        .captures_iter(&input[0])
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut translation_tables = Vec::new();
    let mut i = 3;

    while i < input.len() {
        let mut translation_table = Vec::new();

        while !input[i].is_empty() {
            let range = number_re
                .captures_iter(&input[i])
                .map(|cap| cap[1].parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            translation_table.push(range);
            i += 1;
        }

        translation_tables.push(translation_table);
        i += 2;
    }

    let mut current_ranges = HashSet::new();

    for chunk in seed_numbers.chunks(2) {
        current_ranges.insert((chunk[0], chunk[0] + chunk[1] - 1));
    }

    for table in translation_tables.iter() {
        let mut table_splits = HashSet::new();

        for range in current_ranges.iter() {
            let mut range_splits = HashSet::new();
            range_splits.insert(*range);

            for line in table.iter() {
                let mut line_splits = HashSet::new();

                for range_split in range_splits.iter() {
                    if range_split.1 > line[1] && line[1] + line[2] >= range_split.1
                        || line[1] + line[2] < range_split.1 && range_split.0 < line[1] + line[2]
                    {
                        if range_split.0 < line[1] {
                            line_splits.insert((range_split.0, line[1] - 1));
                        }

                        let overlap = (
                            range_split.0.max(line[1]),
                            range_split.1.min(line[1] + line[2] - 1),
                        );
                        table_splits.insert((
                            line[0] + (overlap.0 - line[1]),
                            line[0] + (overlap.1 - line[1]),
                        ));

                        if range_split.1 > line[1] + line[2] - 1 {
                            line_splits.insert((line[1] + line[2] - 1, range_split.1));
                        }
                    } else {
                        line_splits.insert(*range_split);
                    }
                }

                range_splits = line_splits.clone();
                line_splits.clear();
            }

            table_splits.extend(range_splits.clone());
            range_splits.clear();
        }

        current_ranges = table_splits.clone();
        table_splits.clear();
    }

    println!(
        "Result Part 2: {}",
        current_ranges.iter().map(|x| x.0).min().unwrap()
    );
}
