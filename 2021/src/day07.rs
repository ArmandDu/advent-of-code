pub fn part1() {
    println!("day 7 part 1: v0({})", v0::part1());
}
pub fn part2() {
    println!("day 7 part 2: v0({})", v0::part2());
}

mod v0 {
    use itertools::Itertools;
    use std::collections::HashMap;

    fn get_input() -> HashMap<i32, i32> {
        include_str!("../data/day07_input")
            .trim()
            .split(",")
            .map(|input| input.parse::<i32>().unwrap())
            .fold(HashMap::new(), |mut pool, pos| {
                *pool.entry(pos).or_insert(0) += 1;
                pool
            })
    }

    pub fn part1() -> usize {
        let sorted_distance = include_str!("../data/day07_input")
            .trim()
            .split(",")
            .map(|input| input.parse::<i32>().unwrap())
            .sorted()
            .collect::<Vec<_>>();

        let median_index = (sorted_distance.len() + 1) / 2;
        let median_pos = sorted_distance.get(median_index).unwrap();

        let fuel_cost = sorted_distance
            .iter()
            .map(|pos| (pos - median_pos).abs())
            .sum::<i32>();

        fuel_cost as usize
    }

    pub fn part2() -> usize {
        let crabs: HashMap<_, _> = get_input();

        let (_pos, fuel_cost) = (*crabs.keys().min().unwrap()..*crabs.keys().max().unwrap())
            .map(|target_pos| {
                (
                    target_pos,
                    crabs
                        .iter()
                        .map(|(crab_pos, crab_count)| {
                            let dist = (crab_pos - target_pos).abs();

                            ((dist * (dist + 1)) / 2) * crab_count
                        })
                        .sum::<i32>(),
                )
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();

        fuel_cost as usize
    }
}
