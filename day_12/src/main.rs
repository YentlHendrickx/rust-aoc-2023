use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let parsed = parse_input(&input, false);
    let part_1 = parts(&parsed);
    println!("Part 1: {}", part_1);

    let parsed = parse_input(&input, true);
    let part_2 = parts(&parsed);
    println!("Part 2: {}", part_2);
}

fn parts(lines: &Vec<(Vec<char>, Vec<u64>)>) -> u64 {
    let mut total_ways = 0;

    for (dots, blocks) in lines {
        let way = solve(&dots, &blocks, 0, 0, 0, &mut HashMap::new());

        total_ways += way;
    }

    return total_ways;
}

fn solve(
    dots: &Vec<char>,
    blocks: &Vec<u64>,
    c_pos: u64,
    c_block: u64,
    c_block_length: u64,
    cache_table: &mut HashMap<(u64, u64, u64), u64>,
) -> u64 {
    let key = (c_pos, c_block, c_block_length);

    if cache_table.contains_key(&key) {
        return *cache_table.get(&key).unwrap();
    }

    if c_pos == dots.len() as u64 {
        if c_block == blocks.len() as u64 && c_block_length == 0 {
            return 1;
        } else if c_block == (blocks.len() as u64) - 1 && blocks[c_block as usize] == c_block_length
        {
            return 1;
        } else {
            return 0;
        }
    }

    let mut ways = 0;
    for c in ['.', '#'].iter() {
        if dots[c_pos as usize] == *c || dots[c_pos as usize] == '?' {
            if *c == '.' && c_block_length == 0 {
                ways += solve(&dots, &blocks, c_pos + 1, c_block, 0, cache_table);
            } else if *c == '.'
                && c_block_length > 0
                && c_block < blocks.len() as u64
                && blocks[c_block as usize] == c_block_length
            {
                ways += solve(&dots, &blocks, c_pos + 1, c_block + 1, 0, cache_table);
            } else if *c == '#' {
                ways += solve(
                    &dots,
                    &blocks,
                    c_pos + 1,
                    c_block,
                    c_block_length + 1,
                    cache_table,
                );
            }
        }
    }

    cache_table.insert(key, ways);
    return ways;
}

fn parse_input(input: &str, part_two: bool) -> Vec<(Vec<char>, Vec<u64>)> {
    let mut result: Vec<(Vec<char>, Vec<u64>)> = Vec::new();

    for line in input.lines() {
        // Split the line into two parts: dots and blocks
        let parts: Vec<&str> = line.split(' ').collect();
        let dots: Vec<char> = parts[0].chars().collect();

        // Split the blocks part by commas and parse each part as a number
        let blocks: Vec<u64> = parts[1]
            .split(',')
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();

        result.push((dots, blocks));
    }

    if part_two {
        // For each line the dots should now be 5*dots and the blocks should be 5*blocks
        for (dots, blocks) in result.iter_mut() {
            let mut new_dots: Vec<char> = Vec::new();
            let mut new_blocks: Vec<u64> = Vec::new();

            for i in 0..5 {
                // Extend the dots by 5 times + adda a '?' in between
                new_dots.extend(dots.iter());
                if i == 4 {
                    continue;
                }
                new_dots.push('?');
            }

            for _ in 0..5 {
                new_blocks.extend(blocks.iter());
            }

            *dots = new_dots;
            *blocks = new_blocks;
        }
    }

    return result;
}
