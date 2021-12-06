fn get_content() -> Vec<Vec<usize>> {
    include_str!("../data/day03_star1_input")
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

pub fn part1() {
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

    println!("day 3 part 1: {}", gamma_rate * epsilon_rate);
    assert_eq!(gamma_rate * epsilon_rate, part1_v2());
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

pub fn part2() {
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

    println!("day 3 part 2: {}", oxygen * co2);
}
