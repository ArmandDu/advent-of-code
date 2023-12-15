use aoc::{example, Solution};
use itertools::Itertools;

struct Day15;

#[derive(Debug)]
enum Seq {
    Rm,
    Add(usize),
}

fn hash(input: &str) -> u16 {
    input
        .chars()
        .fold(0, |hash, ascii| (hash + ascii as u16) * 17 % 256)
}

impl Solution for Day15 {
    const TITLE: &'static str = "";
    const DAY: u8 = 15;
    type Input = Vec<String>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        Ok(input.trim().split(',').map(|c| c.to_owned()).collect_vec())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        input.iter().map(|code| hash(code) as usize).sum1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        const BOX: Vec<(String, usize)> = vec![];

        input
            .iter()
            .map(|c| {
                if c.contains('=') {
                    let (code, length) = c.split_once('=')?;
                    let length = length.parse().ok()?;

                    Some((code.to_owned(), Seq::Add(length)))
                } else {
                    Some((c.replace('-', ""), Seq::Rm))
                }
            })
            .fold_options([BOX; 256], |mut boxes, (seq, op)| {
                let index = hash(&seq) as usize;

                match op {
                    Seq::Rm => {
                        let current = boxes[index]
                            .iter()
                            .filter(|(c, _)| c != &seq)
                            .cloned()
                            .collect();

                        boxes[index] = current;
                    }
                    Seq::Add(len) => {
                        if let Some((j, _)) = boxes[index].iter().find_position(|(s, _)| s == &seq)
                        {
                            boxes[index][j] = (seq, len);
                        } else {
                            boxes[index].push((seq, len))
                        }
                    }
                }

                boxes
            })
            .map(|boxes| {
                boxes
                    .iter()
                    .zip(1..)
                    .flat_map(|(lenses, i)| {
                        lenses.iter().zip(1..).map(move |((_, len), j)| i * j * len)
                    })
                    .sum()
            })
    }
}

aoc::run!(Day15);

example! {
    [Day15]
    example: "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
        => Some(1320)
        => Some(145)
}
