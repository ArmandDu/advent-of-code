pub fn part1() {
    println!(
        "day 3 part 1: v0({}) v1({}) v2({})",
        v0::part1(),
        v1::part1(),
        v2::part1()
    );
}
pub fn part2() {
    println!(
        "day 3 part 2: v0({}) v1({}) v2({})",
        v0::part2(),
        v1::part2(),
        v2::part2()
    );
}

mod v2 {
    use crate::day03::v1;

    fn get_input() -> (usize, Vec<usize>) {
        let input = include_str!("../data/day03_input");
        let length = input.find("\n").unwrap();

        let pool = input
            .lines()
            .filter_map(|line| usize::from_str_radix(&line, 2).ok())
            .collect();

        (length, pool)
    }

    fn get_max_byte_at(shift: usize, pool: &[usize]) -> bool {
        pool.iter().map(|value| (value >> shift) & 1).sum::<usize>() * 2 >= pool.len()
    }

    pub fn part1() -> usize {
        let (length, content) = get_input();

        let gamma_rate: usize = (0..length)
            .map(|shift| (get_max_byte_at(shift, &content) as usize) << shift)
            .sum();
        let epsilon_rate = 2_usize.pow(length as u32) - 1 - gamma_rate;

        assert_eq!(gamma_rate * epsilon_rate, v1::part1());
        gamma_rate * epsilon_rate
    }

    pub fn part2() -> usize {
        let (length, content) = get_input();

        let (oxygen, co2): (Vec<_>, Vec<_>) = (0..length).rev().fold(
            (content.to_vec(), content),
            |(mut oxygen, mut co2), shift| {
                if oxygen.len() > 1 {
                    let bit_oxygen = get_max_byte_at(shift, &oxygen) as usize;

                    oxygen.retain(|num| (num >> shift) & 1 == bit_oxygen);
                }

                if co2.len() > 1 {
                    let bit_co2 = !get_max_byte_at(shift, &co2) as usize;

                    co2.retain(|num| (num >> shift) & 1 == bit_co2);
                }

                (oxygen, co2)
            },
        );

        let oxygen = oxygen[0];
        let co2 = co2[0];

        assert_eq!(v1::part2(), oxygen * co2);
        oxygen * co2
    }
}

mod v1 {
    use crate::day03::v0;

    pub fn part1() -> usize {
        let v0_res = v0::part1();
        let content: Vec<&str> = include_str!("../data/day03_input").lines().collect();

        let gamma_rate: String = (0..content.first().unwrap().len())
            .map(|pos| {
                content.iter().fold(String::new(), |mut byte, &row| {
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

        assert_eq!(gamma_rate * epsilon_rate, v0_res);
        gamma_rate * epsilon_rate
    }

    pub fn part2() -> usize {
        let v0_res = v0::part2();
        let content: Vec<String> = include_str!("../data/day03_input")
            .lines()
            .map(|line| String::from(line))
            .collect();

        let oxygen = find_recur(&content, 0, |ones, zeroes| {
            match ones.len() >= zeroes.len() {
                true => ones,
                _ => zeroes,
            }
        });
        let co2 = find_recur(&content, 0, |ones, zeroes| {
            match ones.len() < zeroes.len() {
                true => ones,
                _ => zeroes,
            }
        });
        let oxygen = usize::from_str_radix(&oxygen.unwrap(), 2).unwrap();
        let co2 = usize::from_str_radix(&co2.unwrap(), 2).unwrap();

        assert_eq!(oxygen * co2, v0_res);
        oxygen * co2
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

/// INITIAL submission below
mod v0 {
    fn get_content() -> Vec<Vec<usize>> {
        include_str!("../data/day03_input")
            .lines()
            .map(|line| {
                line.split("")
                    .filter_map(|c| c.parse::<usize>().ok())
                    .collect()
            })
            .collect()
    }

    fn byte_array_to_usize(arr: &[usize]) -> usize {
        let (count, _) = arr
            .iter()
            .rev()
            .fold((0, 1), |(count, exp), b| (count + b * exp, exp * 2));

        count
    }

    pub fn part1_v2() -> usize {
        let content = get_content();

        let flip_content: Vec<Vec<usize>> = (0..content.first().unwrap().len())
            .map(|pos| {
                content.iter().fold(vec![], |mut acc, row| {
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

        byte_array_to_usize(&gamma_rate) * byte_array_to_usize(&epsilon_rate)
    }

    pub fn part1() -> usize {
        let content = get_content();
        let len = content.first().unwrap().len();
        let mut gamma_rate: Vec<usize> = vec![];

        //first attempt, imperative version
        for i in 0..len {
            let mut count1: usize = 0;

            for j in 0..content.len() {
                count1 += content.get(j).unwrap().get(i).unwrap();
            }

            if count1 > content.len() / 2 {
                gamma_rate.push(1)
            } else {
                gamma_rate.push(0)
            }
        }

        let epsilon_rate: Vec<usize> = gamma_rate.clone().iter().map(|b| 1 - b).collect();

        let gamma_rate = byte_array_to_usize(&gamma_rate);
        let epsilon_rate = byte_array_to_usize(&epsilon_rate);

        assert_eq!(gamma_rate * epsilon_rate, part1_v2());

        gamma_rate * epsilon_rate
    }

    fn find_recur<F>(arr: Vec<Vec<usize>>, index: usize, mut candidate: F) -> Option<Vec<usize>>
    where
        F: FnMut(Vec<Vec<usize>>, Vec<Vec<usize>>) -> Vec<Vec<usize>>,
    {
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

                return find_recur(candidate(ones, zeroes), index + 1, candidate);
            }
        };
    }

    pub fn part2() -> usize {
        let content = get_content();

        let oxygen = find_recur(content.clone(), 0, |ones, zeroes| {
            if ones.len() >= zeroes.len() {
                ones
            } else {
                zeroes
            }
        })
        .unwrap();
        let co2 = find_recur(content.clone(), 0, |ones, zeroes| {
            if ones.len() < zeroes.len() {
                ones
            } else {
                zeroes
            }
        })
        .unwrap();

        let oxygen = byte_array_to_usize(&oxygen);
        let co2 = byte_array_to_usize(&co2);

        oxygen * co2
    }
}
