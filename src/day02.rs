use itertools::Itertools;
use std::iter::Map;
use std::str::Lines;

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Aim(i32),
}

fn get_content() -> Vec<Instruction> {
    include_str!("../data/day02_part1_input")
        .lines()
        .filter_map(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();

            let value: i32 = pair[1].parse().unwrap();

            return match pair[0] {
                "down" => Some(Instruction::Aim(value)),
                "up" => Some(Instruction::Aim(-value)),
                "forward" => Some(Instruction::Forward(value)),
                _ => None,
            };
        })
        .collect()
}

pub fn part1() {
    let content = get_content();
    let mut x = 0;
    let mut y = 0;

    for instruction in content {
        match instruction {
            Instruction::Forward(xi) => x += xi,
            Instruction::Aim(yi) => y += yi,
        }
    }

    println!("day 2 part 1: {}", x * y);
}

pub fn part2() {
    let content = get_content();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for instruction in content {
        match instruction {
            Instruction::Forward(xi) => {
                x += xi;
                y += aim * xi;
            }
            Instruction::Aim(yi) => aim += yi,
        }
    }

    println!("day 2 part 2: {}", x * y);
}
