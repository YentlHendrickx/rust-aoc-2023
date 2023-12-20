use std::fs;

#[derive(Debug, Clone)]
struct Rule {
    rule_in: char,
    rule_out: String,
    threshold: u128,
    comparison: char,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    id: String,
    final_out: String,
}

#[derive(Debug)]
struct Rating {
    x: u128,
    m: u128,
    a: u128,
    s: u128,
}

impl Workflow {
    fn process_rating(&self, rating: &Rating) -> String {
        for rule in self.rules.iter() {
            // based on what value of rule, pick the correct rating
            let comp = match rule.rule_in {
                'x' => rating.x,
                'm' => rating.m,
                'a' => rating.a,
                's' => rating.s,
                'e' => continue,
                _ => panic!("Unknown rule_in: {}", rule.rule_in),
            };

            if rule.comparison == '>' {
                if comp > rule.threshold {
                    return rule.rule_out.clone();
                }
            } else if rule.comparison == '<' {
                if comp < rule.threshold {
                    return rule.rule_out.clone();
                }
            }
        }

        return self.final_out.clone();
    }
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let (workflow, rating) = parse(&input);

    let part_1 = part_1(&workflow, rating);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&workflow);
    println!("Part 2: {}", part_2);
}

fn part_2(workflows: &Vec<Workflow>) -> u128 {
    return recurse(
        &workflows.clone(),
        "in",
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
    );
}
fn recurse(
    workflows: &Vec<Workflow>,
    current_id: &str,
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
) -> u128 {
    // Base case for explicit acceptance
    if current_id == "A" {
        return calculate_product(x, m, a, s);
    } else if current_id == "R" {
        return 0;
    }

    let current_workflow = workflows
        .iter()
        .find(|w| w.id == current_id)
        .expect("No workflow with id found");

    // println!(
    //     "Current Workflow: {}, Ranges: x: {:?}, m: {:?}, a: {:?}, s: {:?}",
    //     current_id, x, m, a, s
    // );

    let mut result: u128 = 0;

    // Its terrible and i love it!
    let mut new_x_2 = x;
    let mut new_m_2 = m;
    let mut new_a_2 = a;
    let mut new_s_2 = s;
    let mut new_x_1 = x;
    let mut new_m_1 = m;
    let mut new_a_1 = a;
    let mut new_s_1 = s;

    for rule in &current_workflow.rules {
        if rule.comparison == '>' {
            if rule.rule_in == 'x' {
                new_x_1.0 = rule.threshold as u32 + 1;
                new_x_2.1 = rule.threshold as u32;
            } else if rule.rule_in == 'm' {
                new_m_1.0 = rule.threshold as u32 + 1;
                new_m_2.1 = rule.threshold as u32;
            } else if rule.rule_in == 'a' {
                new_a_1.0 = rule.threshold as u32 + 1;
                new_a_2.1 = rule.threshold as u32;
            } else if rule.rule_in == 's' {
                new_s_1.0 = rule.threshold as u32 + 1;
                new_s_2.1 = rule.threshold as u32;
                if new_x_1.0 > new_x_1.1 {
                    new_x_1.0 = new_x_1.1;
                    new_x_2.1 = new_x_2.0;
                }
            }
        } else {
            if rule.rule_in == 'x' {
                new_x_1.1 = rule.threshold as u32 - 1;
                new_x_2.0 = rule.threshold as u32;
            } else if rule.rule_in == 'm' {
                new_m_1.1 = rule.threshold as u32 - 1;
                new_m_2.0 = rule.threshold as u32;
            } else if rule.rule_in == 'a' {
                new_a_1.1 = rule.threshold as u32 - 1;
                new_a_2.0 = rule.threshold as u32;
            } else if rule.rule_in == 's' {
                new_s_1.1 = rule.threshold as u32 - 1;
                new_s_2.0 = rule.threshold as u32;
            }
        }

        result += recurse(
            workflows,
            &rule.rule_out,
            new_x_1,
            new_m_1,
            new_a_1,
            new_s_1,
        );

        new_x_1 = new_x_2;
        new_m_1 = new_m_2;
        new_a_1 = new_a_2;
        new_s_1 = new_s_2;
    }

    result += recurse(
        workflows,
        &current_workflow.final_out,
        new_x_2,
        new_m_2,
        new_a_2,
        new_s_2,
    );

    return result;
}

// Helper function to calculate the product of the ranges
fn calculate_product(x: (u32, u32), m: (u32, u32), a: (u32, u32), s: (u32, u32)) -> u128 {
    ((x.1 - x.0 + 1) as u128)
        * ((m.1 - m.0 + 1) as u128)
        * ((a.1 - a.0 + 1) as u128)
        * ((s.1 - s.0 + 1) as u128)
}

fn part_1(workflows: &Vec<Workflow>, ratings: Vec<Rating>) -> u128 {
    let mut result = 0;
    // Find start workflow, it's the workflow with id 'in'
    let start_flow = workflows
        .iter()
        .find(|&w| w.id == "in")
        .expect("No workflow with id 'in' found");

    for rating in ratings {
        let mut w_result = start_flow.process_rating(&rating);
        while w_result != "A" && w_result != "R" {
            let next_flow = workflows
                .iter()
                .find(|&w| w.id == w_result)
                .expect("No workflow with id found");

            w_result = next_flow.process_rating(&rating);
        }

        if w_result == "A" {
            result += rating.a + rating.m + rating.s + rating.x;
        }
    }

    return result;
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Rating>) {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let workflows_str = parts[0].trim();
    let ratings_str = parts[1].trim();

    let mut workflows = Vec::new();
    for line in workflows_str.lines() {
        let mut parts = line.split('{');
        let id = parts.next().unwrap().to_string();
        let rules_str = parts.next().unwrap().trim_end_matches('}');
        let mut rules = Vec::new();

        let rule_parts: Vec<&str> = rules_str.split(',').collect();
        let last_rule = rule_parts.len() - 1;

        for (i, rule_str) in rule_parts.iter().enumerate() {
            if i == last_rule {
                continue;
            }
            let parts: Vec<&str> = rule_str.split(':').collect();
            let conditions = parts[0];
            let outcome = parts[1];

            let (_comparison, compare_part) = conditions.split_at(1);
            let (compare_char, threshold) = compare_part.split_at(1);

            let rule = Rule {
                rule_in: conditions.chars().next().unwrap(),
                rule_out: outcome.to_string(),
                threshold: threshold.parse::<u128>().unwrap(),
                comparison: compare_char.chars().next().unwrap(),
            };

            rules.push(rule);
        }

        let workflow = Workflow {
            rules,
            id,
            final_out: rule_parts[last_rule].to_string(),
        };
        workflows.push(workflow);
    }

    let mut ratings = Vec::new();
    for line in ratings_str.split('\n') {
        let parts: Vec<&str> = line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .collect();

        let rating = Rating {
            x: parts[0]
                .trim()
                .split('=')
                .nth(1)
                .unwrap()
                .parse::<u128>()
                .unwrap(),
            m: parts[1]
                .trim()
                .split('=')
                .nth(1)
                .unwrap()
                .parse::<u128>()
                .unwrap(),
            a: parts[2]
                .trim()
                .split('=')
                .nth(1)
                .unwrap()
                .parse::<u128>()
                .unwrap(),

            s: parts[3]
                .trim()
                .split('=')
                .nth(1)
                .unwrap()
                // Remove final char of string
                .trim_end_matches('}')
                .parse::<u128>()
                .unwrap(),
        };
        ratings.push(rating);
    }

    (workflows, ratings)
}
