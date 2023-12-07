use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();

    let part_1 = solve_1(&input, false);
    println!("Part 1: {}", part_1);

    let part_2 = solve_1(&input, true);
    println!("Part 2: {}", part_2)
}

fn solve_1(input: &str, part_two: bool) -> u64 {
    let hands = get_input(input);
    let mut rank: Vec<((&str, u64), u64)> = Vec::new();
    for (_i, hand) in hands.iter().enumerate() {
        let values = count_values(*hand);
        let score = check_score(values.0, part_two);

        // Store card rank and score
        rank.push((*hand, score));
    }

    // Sort the rank by score
    rank.sort_by(|a, b| {
        let score_cmp = b.1.cmp(&a.1);
        if score_cmp == std::cmp::Ordering::Equal {
            (a.0 .0)
                .chars()
                .zip((b.0 .0).chars())
                .map(|(a_card, b_card)| {
                    card_rank_value(b_card, part_two).cmp(&card_rank_value(a_card, part_two))
                })
                .find(|&cmp| cmp != std::cmp::Ordering::Equal)
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            score_cmp
        }
    });

    let mut result = 0;
    for (i, hand) in rank.iter().enumerate() {
        /*    println!(
            "Hand #{}: {:?}, mult: {}",
            i,
            hand,
            (hands.len() - i) as u64
        ); */
        result += (hand.0).1 * (hands.len() - i) as u64;
    }

    return result;
}

fn card_rank_value(card: char, part_two: bool) -> u64 {
    let result = match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u64,
    };

    if part_two && result == 11 {
        return 1;
    }

    return result;
}

fn check_score(card_map: HashMap<char, u64>, part_two: bool) -> u64 {
    // First five of a kind
    if part_two {
        // Find positions of 'J's in the hand
        let j_positions = card_map.get(&'J').cloned().unwrap_or(0);

        if j_positions > 0 {
            let mut best_score = 0;

            // Generate combinations for 'J's
            for card in ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'].iter() {
                let mut temp_map = card_map.clone();
                *temp_map.entry('J').or_insert(0) -= 1; // Reduce one 'J'
                *temp_map.entry(*card).or_insert(0) += 1; // Replace it with the current card

                // Recursively call the function to calculate the score for this combination
                let score = check_score(temp_map, j_positions > 1); // If there are more 'J's, continue replacing

                if score > best_score {
                    best_score = score;
                }
            }
            return best_score;
        }
    }
    let mut top_count = 0;

    for (_card, count) in card_map.iter() {
        if top_count < *count {
            top_count = *count;
        }
    }

    if top_count == 5 {
        return 7;
    }

    if top_count == 4 {
        return 6;
    }

    // Full house
    let mut has_three = false;
    let mut has_two = false;
    for (_card, count) in card_map.iter() {
        if *count == 3 {
            has_three = true;
        } else if *count == 2 {
            has_two = true;
        }
    }

    if has_three && has_two {
        return 5;
    }

    // Three of a kind
    if top_count == 3 {
        return 4;
    }

    // Two or one pair
    let mut pairs = 0;
    for (_card, count) in card_map.iter() {
        if *count == 2 {
            pairs += 1;
        }
    }

    if pairs == 2 {
        return 3;
    } else if pairs == 1 {
        return 2;
    }

    return 0;
}

fn count_values(hand: (&str, u64)) -> (HashMap<char, u64>, Vec<char>) {
    let mut card_map = HashMap::<char, u64>::new();
    let mut cards = Vec::new();

    for card in hand.0.chars() {
        card_map.insert(card, card_map.get(&card).unwrap_or(&0) + 1);
        cards.push(card);
    }

    return (card_map, cards);
}

fn get_input(input: &str) -> Vec<(&str, u64)> {
    return input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|hand| {
            (
                hand[0],
                hand[1].parse::<u64>().expect("Could not parse card value"),
            )
        })
        .collect::<Vec<(&str, u64)>>();
}
