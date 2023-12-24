use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Block {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
    z0: usize,
    z1: usize,
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let blocks = parse_input(&input);

    let new_blocks = fall_down(&blocks);
    // print_blocks(&new_blocks, false);
    // print_blocks(&new_blocks, false);

    let support_map = build_support_map(&new_blocks);
    let part_1 = part_1(&new_blocks, &support_map);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&new_blocks, &support_map);
    println!("Part 2: {}", part_2);
}

fn part_1(blocks: &Vec<Block>, support_map: &HashMap<Block, Vec<Block>>) -> u32 {
    let mut result = 0;

    for i in 0..blocks.len() {
        let current_block = blocks[i].clone();
        if can_be_safely_removed(current_block, &support_map) {
            result += 1;
        }
    }

    return result;
}

fn part_2(blocks: &Vec<Block>, support_map: &HashMap<Block, Vec<Block>>) -> u32 {
    let mut result = 0;

    // Sort by z0
    let mut blocks = blocks.clone();
    blocks.sort_by(|a, b| a.z0.cmp(&b.z0));

    for block in blocks {
        let mut queue: LinkedList<(Block, Block)> = LinkedList::new();
        let mut removed_b: HashSet<Block> = HashSet::new();
        let mut support_map = support_map.clone();
        // Append all supported blocks to queue
        if let Some(supported_blocks) = support_map.get(&block) {
            for supported_block in supported_blocks {
                queue.push_back((supported_block.clone(), block));
            }

            support_map.remove(&block);
        }

        while queue.len() > 0 {
            let (c_block, removed) = queue.pop_front().unwrap();

            if removed_b.contains(&c_block) {
                continue;
            }

            if !has_alternative_support(&c_block, &removed, &support_map) {
                result += 1;
                removed_b.insert(c_block.clone());

                // Append all supported blocks to queue
                if let Some(supported_blocks) = support_map.get(&c_block) {
                    for supported_block in supported_blocks {
                        if removed_b.contains(supported_block) {
                            continue;
                        }

                        queue.push_back((supported_block.clone(), c_block.clone()));
                    }
                }
                support_map.remove(&c_block);
            }
        }
    }

    return result;
}

fn can_be_safely_removed(block: Block, support_map: &HashMap<Block, Vec<Block>>) -> bool {
    if !support_map.contains_key(&block) {
        return true;
    }
    if let Some(supported_blocks) = support_map.get(&block) {
        for &supported_block in supported_blocks {
            if !has_alternative_support(&supported_block, &block, support_map) {
                return false;
            }
        }
    }
    return true;
}
fn has_alternative_support(
    block: &Block,
    removed_block: &Block,
    support_map: &HashMap<Block, Vec<Block>>,
) -> bool {
    // Iterate through the support map to find if any other block supports the given block
    for (supporting_block, supported_blocks) in support_map {
        if supporting_block != removed_block && supported_blocks.contains(block) {
            return true; // Found an alternative supporting block
        }
    }

    return false;
}

fn build_support_map(blocks: &[Block]) -> HashMap<Block, Vec<Block>> {
    let mut support_map = HashMap::new();

    for block in blocks {
        for other_block in blocks {
            if block == other_block {
                continue;
            }

            if block.z1 + 1 == other_block.z0 {
                let block_x_range = block.x0..block.x1;
                let block_y_range = block.y0..block.y1;
                let other_x_range = other_block.x0..other_block.x1;
                let other_y_range = other_block.y0..other_block.y1;

                if check_overlapping_range(&block_x_range, &other_x_range)
                    && check_overlapping_range(&block_y_range, &other_y_range)
                {
                    // `block` is supporting `other_block`
                    support_map
                        .entry(block.clone())
                        .or_insert_with(Vec::new)
                        .push(other_block.clone());
                }
            }
        }
    }
    return support_map;
}

fn fall_down(blocks: &Vec<Block>) -> Vec<Block> {
    let mut new_blocks = blocks.clone();

    loop {
        let mut moved = false;
        new_blocks.sort_by(|a, b| a.z0.cmp(&b.z0));

        for i in 0..new_blocks.len() {
            let mut block = new_blocks[i];
            let can_move = can_move_down(&block, &new_blocks);
            if can_move {
                block.z0 -= 1;
                block.z1 -= 1;
                // Update block at index
                new_blocks[i] = block.clone();
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }

    return new_blocks;
}

fn can_move_down(block: &Block, blocks: &Vec<Block>) -> bool {
    for other_block in blocks {
        if other_block == block || block.z0 <= 1 {
            if block.z0 <= 1 {
                return false;
            }
            continue;
        }

        if block.z0 - 1 == other_block.z1 {
            let block_x_range = block.x0..block.x1;
            let block_y_range = block.y0..block.y1;
            let other_x_range = other_block.x0..other_block.x1;
            let other_y_range = other_block.y0..other_block.y1;

            if check_overlapping_range(&other_x_range, &block_x_range)
                && check_overlapping_range(&other_y_range, &block_y_range)
            {
                return false;
            }
        }
    }
    return true;
}

fn check_overlapping_range(
    range1: &std::ops::Range<usize>,
    range2: &std::ops::Range<usize>,
) -> bool {
    return range1.start <= range2.end && range2.start <= range1.end;
}

fn print_blocks(blocks: &Vec<Block>, front_view: bool) {
    // Print grid, either x,z or y,z
    // If front_view we use x as the x axis and z as the y axis
    // If not we use y as the x axis and z as the y axis
    let mut max_x = 0;

    if front_view {
        for block in blocks.iter() {
            if block.x1 > max_x {
                max_x = block.x1;
            }
        }
    } else {
        for block in blocks.iter() {
            if block.y1 > max_x {
                max_x = block.y1;
            }
        }
    }

    let mut max_z = 0;
    for block in blocks.iter() {
        if block.z1 > max_z {
            max_z = block.z1;
        }
    }

    let mut grid = vec![vec!['.'; max_x + 1]; max_z + 1];

    // Sort blocks on z0
    for (i, block) in blocks.iter().enumerate() {
        if front_view {
            for x in block.x0..=block.x1 {
                for z in block.z0..=block.z1 {
                    // Starting at A in ASCII table + i
                    grid[z][x] = (65 + i) as u8 as char;
                }
            }
        } else {
            for y in block.y0..=block.y1 {
                for z in block.z0..=block.z1 {
                    grid[z][y] = (65 + i) as u8 as char;
                }
            }
        }
    }

    // Print but flip the y axis
    for x in 0..=grid[0].len() - 1 {
        print!("{}", x);
    }
    print!("\n");
    for (r, row) in grid.iter().rev().enumerate() {
        for col in row.iter() {
            print!("{}", col);
        }
        print!(" {}", grid.len() - r - 1);
        print!("\n");
    }
    println!("\n");
}

fn parse_input(input: &str) -> Vec<Block> {
    let lines = input.lines();

    let mut blocks = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split("~").collect();
        let first = parts[0].split(",").collect::<Vec<&str>>();
        let second = parts[1].split(",").collect::<Vec<&str>>();

        let block = Block {
            x0: first[0].parse::<usize>().unwrap(),
            x1: second[0].parse::<usize>().unwrap(),
            y0: first[1].parse::<usize>().unwrap(),
            y1: second[1].parse::<usize>().unwrap(),
            z0: first[2].parse::<usize>().unwrap(),
            z1: second[2].parse::<usize>().unwrap(),
        };

        blocks.push(block);
    }

    return blocks;
}
