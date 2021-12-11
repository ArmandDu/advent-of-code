pub fn part1() {
    println!("day 6 part 1: v0({}) v1({})", v0::part1(), v1::part1());
}
pub fn part2() {
    println!("day 6 part 2: v0({}) v1({})", v0::part2(), v1::part2());
}

mod v1 {
    use crate::day06::v0;

    fn get_input() -> [usize; 9] {
        include_str!("../data/day06_input")
            .split(",")
            .filter_map(|i| i.parse::<usize>().ok())
            .fold([0; 9], |mut map, i| {
                map[i] += 1;
                map
            })
    }

    fn making_babies(pool: &[usize], days: usize) -> usize {
        (0..days)
            .fold(pool.to_vec(), |mut pool, _day| {
                let new_babies = pool[0];

                pool.rotate_left(1);
                pool[6] += new_babies;

                pool
            })
            .iter()
            .sum()
    }

    pub fn part1() -> usize {
        let input = get_input();
        let population = making_babies(&input, 80);

        assert_eq!(v0::part1(), population);
        population
    }

    pub fn part2() -> usize {
        let input = get_input();
        let population = making_babies(&input, 256);

        population
    }
}

mod v0 {
    fn get_population(initial: &[usize], target: usize) -> Vec<usize> {
        (0..target).fold(initial.into(), |pool, _day| {
            let mut newborns: Vec<usize> = vec![];

            let mut pool: Vec<usize> = pool
                .iter()
                .map(|&life| {
                    if life == 0 {
                        newborns.push(8);
                        6
                    } else {
                        life - 1
                    }
                })
                .collect();

            pool.append(&mut newborns);
            pool
        })
    }

    pub fn part1() -> usize {
        let initial: Vec<usize> = include_str!("../data/day06_input")
            .split(",")
            .filter_map(|i| i.parse::<usize>().ok())
            .collect();

        let target = 80usize;
        let pool = get_population(&initial, target);

        pool.len()
    }

    //this would  be a bad idea :D
    pub fn part2() -> usize {
        // let initial: Vec<usize> = include_str!("../data/day06_input")
        //     .split(",")
        //     .filter_map(|i| i.parse::<usize>().ok())
        //     .collect();

        // simulation would take to long. need to fix
        // let target = 256usize;
        // let pool = get_population(&initial, target);

        0
    }
}
