use year2021::Solution;

pub mod v2 {
    use year2021::Solution;

    pub struct Day03;

    impl Solution<usize, usize> for Day03 {
        const DAY: u32 = 3;
        const YEAR: u32 = 2021;
        const TITLE: &'static str = "Binary Diagnostic (V2)";
        type Input = (usize, Vec<usize>);

        fn part1(input: &Self::Input) -> Option<usize> {
            let (length, content) = input.to_owned();

            let gamma_rate: usize = (0..length)
                .map(|shift| (Self::get_max_byte_at(shift, &content) as usize) << shift)
                .sum();
            let epsilon_rate = 2_usize.pow(length as u32) - 1 - gamma_rate;

            Some(gamma_rate * epsilon_rate)
        }

        fn part2(input: &Self::Input) -> Option<usize> {
            let (length, content) = input.to_owned();

            let (oxygen, co2) = (0..length).rev().fold(
                (content.to_vec(), content),
                |(mut oxygen, mut co2), shift| {
                    if oxygen.len() > 1 {
                        let bit_oxygen = Self::get_max_byte_at(shift, &oxygen) as usize;

                        oxygen.retain(|num| (num >> shift) & 1 == bit_oxygen);
                    }

                    if co2.len() > 1 {
                        let bit_co2 = !Self::get_max_byte_at(shift, &co2) as usize;

                        co2.retain(|num| (num >> shift) & 1 == bit_co2);
                    }

                    (oxygen, co2)
                },
            );

            let oxygen = oxygen[0];
            let co2 = co2[0];

            Some(oxygen * co2)
        }

        fn parse(input: &str) -> Result<Self::Input, &str> {
            let length = input.find("\n").unwrap();

            let pool = input
                .lines()
                .filter_map(|line| usize::from_str_radix(&line, 2).ok())
                .collect();

            Ok((length, pool))
        }
    }

    impl Day03 {
        fn get_max_byte_at(shift: usize, pool: &[usize]) -> bool {
            pool.iter().map(|value| (value >> shift) & 1).sum::<usize>() * 2 >= pool.len()
        }
    }
}

pub mod v1 {
    use year2021::Solution;

    pub struct Day03;

    impl Solution<usize, usize> for Day03 {
        const DAY: u32 = 3;
        const YEAR: u32 = 2021;
        const TITLE: &'static str = "Binary Diagnostic (V1)";
        type Input = Vec<String>;

        fn part1(input: &Self::Input) -> Option<usize> {
            let gamma_rate: String = (0..input.first().unwrap().len())
                .map(|pos| {
                    input.iter().fold(String::new(), |mut byte, row| {
                        let bit: char = *row.chars().collect::<Vec<_>>().get(pos).unwrap();

                        byte.push(bit);
                        byte
                    })
                })
                .map(|row| match row.chars().filter(|&bit| bit == '1').count() {
                    x if x > row.len() / 2 => '1',
                    _ => '0',
                })
                .collect();

            let epsilon_rate: String = gamma_rate
                .chars()
                .map(|b| match b {
                    '0' => '1',
                    _ => '0',
                })
                .collect();

            let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
            let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();

            Some(gamma_rate * epsilon_rate)
        }

        fn part2(input: &Self::Input) -> Option<usize> {
            let oxygen = find_recur(&input, 0, |ones, zeroes| match ones.len() >= zeroes.len() {
                true => ones,
                _ => zeroes,
            });
            let co2 = find_recur(&input, 0, |ones, zeroes| match ones.len() < zeroes.len() {
                true => ones,
                _ => zeroes,
            });
            let oxygen = usize::from_str_radix(&oxygen.unwrap(), 2).unwrap();
            let co2 = usize::from_str_radix(&co2.unwrap(), 2).unwrap();

            Some(oxygen * co2)
        }

        fn parse(input: &str) -> Result<Self::Input, &str> {
            Ok(input
                .trim()
                .lines()
                .map(|line| line.trim().to_string())
                .collect())
        }
    }

    fn find_recur<F>(pool: &Vec<String>, index: usize, mut candidate: F) -> Option<String>
    where
        F: FnMut(Vec<String>, Vec<String>) -> Vec<String>,
    {
        match pool.len() {
            0 => None,
            1 => Some(String::from(pool.first().unwrap())),
            _ => {
                let (ones, zeroes): (Vec<_>, Vec<_>) = pool
                    .iter()
                    .map(|str| str.clone())
                    .partition(|row| row.chars().collect::<Vec<_>>().get(index).unwrap() == &'1');

                find_recur(&candidate(ones, zeroes), index + 1, candidate)
            }
        }
    }
}

pub mod v0_1 {
    use crate::day03::Day03 as Day03V0;
    use year2021::Solution;

    pub struct Day03;

    impl Solution<usize, usize> for Day03 {
        const DAY: u32 = 3;
        const YEAR: u32 = 2021;
        const TITLE: &'static str = "Binary Diagnostic (V0.1)";
        type Input = Vec<Vec<usize>>;

        fn part1(input: &Self::Input) -> Option<usize> {
            let flip_content: Vec<Vec<usize>> = (0..input.first().unwrap().len())
                .map(|pos| {
                    input.iter().fold(vec![], |mut acc, row| {
                        acc.push(row.get(pos).unwrap().clone());

                        acc
                    })
                })
                .collect();

            let gamma_rate: Vec<usize> = flip_content
                .iter()
                .map(|row| (row.iter().sum::<usize>() > row.len() / 2).into())
                .collect();
            let epsilon_rate: Vec<usize> = gamma_rate.iter().map(|b| 1 - b).collect();

            Some(
                Day03V0::byte_array_to_usize(&gamma_rate)
                    * Day03V0::byte_array_to_usize(&epsilon_rate),
            )
        }

        fn part2(_input: &Self::Input) -> Option<usize> {
            None
        }

        fn parse(input: &str) -> Result<Self::Input, &str> {
            Day03V0::parse(input)
        }
    }
}

/// INITIAL submission below (converted to impl Solution trait)
pub struct Day03;

impl Solution<usize, usize> for Day03 {
    const DAY: u32 = 3;
    const YEAR: u32 = 2021;
    const TITLE: &'static str = "Binary Diagnostic (V0)";
    type Input = Vec<Vec<usize>>;

    fn part1(input: &Self::Input) -> Option<usize> {
        let len = input.first().unwrap().len();
        let mut gamma_rate: Vec<usize> = vec![];

        //first attempt, imperative version
        for i in 0..len {
            let mut count1: usize = 0;

            for j in 0..input.len() {
                count1 += input.get(j).unwrap().get(i).unwrap();
            }

            if count1 > input.len() / 2 {
                gamma_rate.push(1)
            } else {
                gamma_rate.push(0)
            }
        }

        let epsilon_rate: Vec<usize> = gamma_rate.clone().iter().map(|b| 1 - b).collect();

        let gamma_rate = Self::byte_array_to_usize(&gamma_rate);
        let epsilon_rate = Self::byte_array_to_usize(&epsilon_rate);

        Some(gamma_rate * epsilon_rate)
    }

    fn part2(input: &Self::Input) -> Option<usize> {
        let oxygen = Self::find_recur(input.clone(), 0, |ones, zeroes| {
            if ones.len() >= zeroes.len() {
                ones
            } else {
                zeroes
            }
        })
        .unwrap();
        let co2 = Self::find_recur(input.clone(), 0, |ones, zeroes| {
            if ones.len() < zeroes.len() {
                ones
            } else {
                zeroes
            }
        })
        .unwrap();

        let oxygen = Self::byte_array_to_usize(&oxygen);
        let co2 = Self::byte_array_to_usize(&co2);

        Some(oxygen * co2)
    }

    fn parse(input: &str) -> Result<Self::Input, &str> {
        Ok(input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .split("")
                    .filter_map(|c| c.parse::<usize>().ok())
                    .collect()
            })
            .collect())
    }
}

impl Day03 {
    fn byte_array_to_usize(arr: &[usize]) -> usize {
        let (count, _) = arr
            .iter()
            .rev()
            .fold((0, 1), |(count, exp), b| (count + b * exp, exp * 2));

        count
    }

    fn find_recur(
        arr: Vec<Vec<usize>>,
        index: usize,
        candidate: impl Fn(Vec<Vec<usize>>, Vec<Vec<usize>>) -> Vec<Vec<usize>>,
    ) -> Option<Vec<usize>> {
        return match arr.len() {
            0 => None,
            1 => Some(arr.first().unwrap().clone()),
            _ => {
                // let (ones, zeroes): (Vec<_>, Vec<_>) = arr.iter().partition(|&row| row.get(index).unwrap() == 1);

                // Haven't figured out how to use the .partition, so do it manually for now
                let (ones, zeroes) =
                    arr.iter()
                        .fold((vec![], vec![]), |(mut ones, mut zeroes), row| {
                            if row.get(index).unwrap() == &1 {
                                ones.push(row.clone())
                            } else {
                                zeroes.push(row.clone())
                            }

                            (ones, zeroes)
                        });

                return Self::find_recur(candidate(ones, zeroes), index + 1, candidate);
            }
        };
    }
}
