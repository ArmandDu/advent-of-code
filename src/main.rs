fn main() {
    day01_part1();
    day01_part2();
}

fn get_content() -> Vec<i32> {
    include_str!("../data/day01_star1_input")
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect()
}

fn day01_part1() {
    let content = get_content();
    let count = content.windows(2).filter(|pair| pair[0] < pair[1]).count();

    println!("day 1 part 1: {}", count);
}

fn day01_part2() {
    let content = get_content();
    let count = content.windows(4).filter(|pair| pair[0] < pair[3]).count();

    println!("day 1 part 2: {}", count);
}
