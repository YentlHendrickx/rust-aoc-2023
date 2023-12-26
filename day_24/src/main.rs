use mathru::algebra::linear::matrix::{General, Solve};
use mathru::algebra::linear::vector::Vector;
use mathru::vector;

use std::fs;
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Stone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug)]
struct Z3Stone<'a> {
    px: Int<'a>,
    py: Int<'a>,
    pz: Int<'a>,
    vx: Int<'a>,
    vy: Int<'a>,
    vz: Int<'a>,
}

impl Stone {
    fn z3_stone<'a>(&self, context: &'a Context) -> Z3Stone<'a> {
        Z3Stone {
            px: Int::from_i64(context, self.px as i64),
            py: Int::from_i64(context, self.py as i64),
            pz: Int::from_i64(context, self.pz as i64),
            vx: Int::from_i64(context, self.vx as i64),
            vy: Int::from_i64(context, self.vy as i64),
            vz: Int::from_i64(context, self.vz as i64),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./data/input.txt").unwrap();
    let stones = parse(&input);

    let part_1 = part_1(&stones);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&stones);
    println!("Part 2: {}", part_2);
}

fn part_1(stones: &Vec<Stone>) -> usize {
    let mut intersections = 0;
    for (i, h1) in (1..).zip(stones) {
        for h2 in &stones[i..] {
            // Matrices
            let a = General::new(2, 2, vec![h1.vx, h1.vy, -h2.vx, -h2.vy]);
            let b = vector![h2.px - h1.px; h2.py - h1.py];

            if let Ok(t) = a.solve(&b) {
                if t[0] >= 0. && t[1] >= 0. {
                    let x1 = h1.px + h1.vx * t[0];
                    let y1 = h1.py + h1.vy * t[0];
                    if x1 >= 200000000000000.
                        && x1 <= 400000000000000.
                        && y1 >= 200000000000000.
                        && y1 <= 400000000000000.
                    {
                        intersections += 1;
                    }
                }
            }
        }
    }

    return intersections;
}

fn part_2(stones: &Vec<Stone>) -> f64 {
    use SatResult::*;

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let rock_x = Int::new_const(&ctx, "rock_x");
    let rock_y = Int::new_const(&ctx, "rock_y");
    let rock_z = Int::new_const(&ctx, "rock_z");
    let rock_vx = Int::new_const(&ctx, "rock_vx");
    let rock_vy = Int::new_const(&ctx, "rock_vy");
    let rock_vz = Int::new_const(&ctx, "rock_vz");
    let zero = Int::from_i64(&ctx, 0);

    for (i, stone) in (0..15).zip(stones.iter().skip(5).map(|s| s.z3_stone(&ctx))) {
        let t = Int::new_const(&ctx, format!("t_{}", i));

        solver.assert(&t.gt(&zero));
        solver.assert(&(&rock_x + &rock_vx * &t)._eq(&(stone.px + stone.vx * &t)));
        solver.assert(&(&rock_y + &rock_vy * &t)._eq(&(stone.py + stone.vy * &t)));
        solver.assert(&(&rock_z + &rock_vz * &t)._eq(&(stone.pz + stone.vz * &t)));
    }

    if let (Sat, Some(model)) = (solver.check(), solver.get_model()) {
        let solution = model.eval(&(rock_x + rock_y + rock_z), true).unwrap();
        return solution.as_i64().unwrap() as f64;
    }

    return 0.;
}

fn parse(input: &str) -> Vec<Stone> {
    let mut stones = Vec::new();

    for line in input.lines() {
        let parts = line.split("@").collect::<Vec<&str>>();
        let mut positions = parts[0].trim().split(",");

        let px = positions.next().unwrap().trim().parse::<f64>().unwrap();
        let py = positions.next().unwrap().trim().parse::<f64>().unwrap();
        let pz = positions.next().unwrap().trim().parse::<f64>().unwrap();

        let mut velocities = parts[1].trim().split(",");
        let vx = velocities.next().unwrap().trim().parse::<f64>().unwrap();
        let vy = velocities.next().unwrap().trim().parse::<f64>().unwrap();
        let vz = velocities.next().unwrap().trim().parse::<f64>().unwrap();

        stones.push(Stone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        });
    }

    return stones;
}
