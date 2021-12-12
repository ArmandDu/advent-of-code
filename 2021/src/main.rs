use year2021::Solution;

mod day01;
mod day02;
mod day03;
mod day06;
mod day07;

fn main() {
    println!("---- TESTING ---\n");
    day01::Day01::test(
        "199\n200\n208\n210\n200\n207\n240\n269\n260\n263",
        Some(7),
        Some(5),
    );
    day02::Day02::test(
        "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
        Some(150),
        Some(900),
    );
    day02::Day02V2::test(
        "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2",
        Some(150),
        Some(900),
    );
    day06::Day06V0::test("3,4,3,1,2", Some(5934), Some(26984457539));
    day06::Day06::test("3,4,3,1,2", Some(5934), Some(26984457539));
    day07::Day07::test("16,1,2,0,4,2,7,1,2,14", Some(37), Some(168));

    println!("\n---- SOLUTIONS ---\n");

    day03::part1();
    day03::part2();

    day01::Day01::run(include_str!("../data/day01_star1_input"));

    day02::Day02::run(include_str!("../data/day02_part1_input"));
    day02::Day02V2::run(include_str!("../data/day02_part1_input"));

    day06::Day06V0::run(include_str!("../data/day06_input"));
    day06::Day06::run(include_str!("../data/day06_input"));

    day07::Day07::run(include_str!("../data/day07_input"));
}
