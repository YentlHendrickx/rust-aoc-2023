fn main() {
    let input = std::fs::read_to_string("./data/input.txt").unwrap();
    let vectorized: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let total_points: u32 = first_half(&vectorized);

    println!("Total points: {}", total_points);
}

fn first_half(vectorized: &Vec<&str>) -> u32 {
    let mut total_points = 0;
    
    for (i, line) in vectorized.iter().enumerate() {
        let colon_index = line.find(':').unwrap();
        let stripped_line = &line[colon_index + 2..].trim();

        let left_numbers = stripped_line.split('|').collect::<Vec<&str>>()[0];
        let right_numbers = stripped_line.split('|').collect::<Vec<&str>>()[1];
       
        let mut winning_numbers = Vec::new();

        for number in left_numbers.trim().split(' ').collect::<Vec<&str>>() {
            if number.is_empty() {
                continue;
            }

            winning_numbers.push(number);
        }


        let mut points = 0;

        for number in right_numbers.trim().split(' ').collect::<Vec<&str>>() {
            if number.is_empty() {
                continue;
            }

            if winning_numbers.contains(&number) {
                
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total_points += points;

    }


    return total_points;
}
