use year2021::Solution;

#[derive(Debug)]
pub enum Instruction {
    Forward(i32),
    Aim(i32),
}

pub struct Day02;
pub struct Day02V2;

impl Solution<i32, i32> for Day02 {
    const DAY: u32 = 2;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Dive!";
    type Input = Vec<Instruction>;

    fn part1(input: &Self::Input) -> Option<i32> {
        let mut x = 0;
        let mut y = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(xi) => x += xi,
                Instruction::Aim(yi) => y += yi,
            }
        }

        Some(x * y)
    }

    fn part2(input: &Self::Input) -> Option<i32> {
        let mut x = 0;
        let mut y = 0;
        let mut aim = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(xi) => {
                    x += xi;
                    y += aim * xi;
                }
                Instruction::Aim(yi) => aim += yi,
            }
        }

        Some(x * y)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(parse(&input))
    }
}

impl Solution<i32, i32> for Day02V2 {
    const DAY: u32 = 2;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Dive! (Implementation V2)";
    type Input = Vec<Instruction>;

    fn part1(input: &Self::Input) -> Option<i32> {
        let (x, y) = input
            .iter()
            .fold((0, 0), |(x, y), instruction| match instruction {
                Instruction::Forward(xi) => (x + xi, y),
                Instruction::Aim(yi) => (x, y + yi),
            });

        Some(x * y)
    }

    fn part2(input: &Self::Input) -> Option<i32> {
        let (x, y, _) =
            input
                .iter()
                .fold((0, 0, 0), |(x, y, aim), instruction| match instruction {
                    Instruction::Forward(xi) => (x + xi, y + (aim * xi), aim),
                    Instruction::Aim(yi) => (x, y, aim + yi),
                });

        Some(x * y)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(parse(&input))
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
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
