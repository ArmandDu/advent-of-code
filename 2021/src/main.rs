use year2021::Solution;

mod day01;
mod day02;
mod day03;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day13;

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

    day03::Day03::test(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        Some(198),
        Some(230),
    );
    day03::v0_1::Day03::test(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        Some(198),
        Some(230),
    );
    day03::v1::Day03::test(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        Some(198),
        Some(230),
    );
    day03::v2::Day03::test(
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        Some(198),
        Some(230),
    );

    day06::Day06V0::test("3,4,3,1,2", Some(5934), Some(26984457539));
    day06::Day06::test("3,4,3,1,2", Some(5934), Some(26984457539));

    day07::Day07::test("16,1,2,0,4,2,7,1,2,14", Some(37), Some(168));

    day08::Day08::test(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", 
        Some(26),
        Some(61229));

    day09::Day09::test(
        "2199943210\n3987894921\n9856789892\n8767896789\n9899965678",
        Some(15),
        Some(1134),
    );

    day10::Day10::test(
        "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]",
        Some(26397),
        Some(288957),
    );

    day13::Day13::test("6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5", Some(17), Some("\n▓▓▓▓▓\n▓   ▓\n▓   ▓\n▓   ▓\n▓▓▓▓▓".to_string()));

    println!("\n---- SOLUTIONS ---\n");

    day01::Day01::run(include_str!("../data/day01_star1_input"));

    day02::Day02::run(include_str!("../data/day02_part1_input"));
    day02::Day02V2::run(include_str!("../data/day02_part1_input"));

    day03::Day03::run(include_str!("../data/day03_input"));
    day03::v0_1::Day03::run(include_str!("../data/day03_input"));
    day03::v1::Day03::run(include_str!("../data/day03_input"));
    day03::v2::Day03::run(include_str!("../data/day03_input"));

    day06::Day06V0::run(include_str!("../data/day06_input"));
    day06::Day06::run(include_str!("../data/day06_input"));

    day07::Day07::run(include_str!("../data/day07_input"));

    day08::Day08::run(include_str!("../data/day08_input"));

    day09::Day09::run(include_str!("../data/day09_input"));

    day10::Day10::run(include_str!("../data/day10_input"));

    day13::Day13::run(include_str!("../data/day13_input"));
}
