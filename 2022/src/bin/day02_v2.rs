use aoc::Solution;
use aoc_utils::lines_to_owned;

struct Day02;

impl Solution for Day02 {
    const TITLE: &'static str = "Rock Paper Scissors (v2)";
    const DAY: u8 = 2;
    type Input = Vec<String>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(lines_to_owned(input))
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .map(|line| {
                    // A || X = ROCK
                    // B || Y = PAPER
                    // C || Z= SCISSOR

                    const WIN_SCORE: usize = 6;
                    const DRAW_SCORE: usize = 3;
                    const LOSE_SCORE: usize = 0;

                    match line.as_str() {
                        "A X" => DRAW_SCORE + 1,
                        "A Y" => WIN_SCORE + 2,
                        "A Z" => LOSE_SCORE + 3,
                        "B X" => LOSE_SCORE + 1,
                        "B Y" => DRAW_SCORE + 2,
                        "B Z" => WIN_SCORE + 3,
                        "C X" => WIN_SCORE + 1,
                        "C Y" => LOSE_SCORE + 2,
                        "C Z" => DRAW_SCORE + 3,
                        _ => unreachable!(),
                    }
                })
                .sum(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        Some(
            input
                .iter()
                .map(|line| {
                    // A = ROCK
                    // B = PAPER
                    // C = SCISSOR

                    // X = lose
                    const X_SCORE: usize = 0;
                    // Y = draw
                    const Y_SCORE: usize = 3;
                    // Z = win
                    const Z_SCORE: usize = 6;

                    match line.as_str() {
                        "A X" => X_SCORE + 3,
                        "A Y" => Y_SCORE + 1,
                        "A Z" => Z_SCORE + 2,
                        "B X" => X_SCORE + 1,
                        "B Y" => Y_SCORE + 2,
                        "B Z" => Z_SCORE + 3,
                        "C X" => X_SCORE + 2,
                        "C Y" => Y_SCORE + 3,
                        "C Z" => Z_SCORE + 1,
                        _ => unreachable!(),
                    }
                })
                .sum(),
        )
    }
}

fn main() {
    aoc::solution!(Day02)
}
#[cfg(test)]
mod tests {
    use crate::Day02 as day_02_v2;
    use crate::*;

    aoc::test_common!(day_02_v2);

    aoc::test! {
        day_02_v2:
        - "A Y\nB X\nC Z"
            => Some(15)
            => Some(12)
    }
}
