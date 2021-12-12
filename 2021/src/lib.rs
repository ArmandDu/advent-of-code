use std::{fmt::Display, time::Instant};

macro_rules! bench {
    ($e:expr) => {{
        let start = Instant::now();
        let result = $e;
        let elapsed = Instant::now().duration_since(start);
        (result, elapsed)
    }};
}

pub trait Solution<P1: Display, P2: Display> {
    const DAY: u32;
    const YEAR: u32;
    const TITLE: &'static str;

    type Input;

    fn part1(input: &Self::Input) -> Option<P1>;
    fn part2(input: &Self::Input) -> Option<P2>;
    fn parse(input: &str) -> Result<Self::Input, &str>;

    fn test(input: &str, part1: Option<P1>, part2: Option<P2>) {
        let mut count = 0;
        let input = Self::parse(&input).unwrap();

        if let (Some(r1), Some(expected)) = (Self::part1(&input), part1) {
            assert_eq!(expected.to_string(), r1.to_string());
            count += 1;
        }

        if let (Some(r2), Some(expected)) = (Self::part2(&input), part2) {
            assert_eq!(expected.to_string(), r2.to_string());
            count += 1;
        }

        println!(
            "{} Day {:02}: {} - {}/2 tests passed!",
            Self::YEAR,
            Self::DAY,
            Self::TITLE,
            count
        );
    }

    fn run(input: &str) {
        let (input, parse_time) = bench!(Self::parse(&input));
        let input = input.unwrap();
        let (p1, s1) = bench!(Self::part1(&input));
        let (p2, s2) = bench!(Self::part2(&input));

        println!("{} Day {:02}: {}", Self::YEAR, Self::DAY, Self::TITLE);

        println!(
            "\tParsing\t\t\t- in {:.4}ms",
            parse_time.as_secs_f32() * 1000.
        );
        if let Some(r1) = p1 {
            println!("\tPart 1: '{}'\t- in {:.4}ms", r1, s1.as_secs_f32() * 1000.);
        }
        if let Some(r2) = p2 {
            println!("\tPart 2: '{}'\t- in {:.4}ms", r2, s2.as_secs_f32() * 1000.);
        }

        println!(
            "\tTotal Time:\t\t{:.4}ms",
            (parse_time.as_secs_f32() + s1.as_secs_f32() + s2.as_secs_f32()) * 1000.
        );
    }
}
