use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::{Chars, FromStr};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand(String, HashMap<char, usize>);

impl Hand {
    fn iter(&self) -> Chars<'_> {
        self.0.chars()
    }

    fn score(&self) -> HashMap<char, usize> {
        self.1.to_owned()
    }

    fn new(input: &str) -> Self {
        let map: HashMap<char, usize> = input.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_default() += 1;
            map
        });

        Self(input.to_owned(), map)
    }
}

#[derive(Debug)]
struct Game(Hand, usize);

impl Game {
    fn hand(&self) -> &Hand {
        &self.0
    }

    fn bid(&self) -> usize {
        self.1
    }
}

struct CamelCard(String, Option<char>);

impl CamelCard {
    fn new(base: &str, joker: Option<char>) -> Self {
        Self(base.to_owned(), joker)
    }

    fn index(&self, card: char) -> Option<usize> {
        self.0.chars().position(|c| c == card)
    }

    fn get_type(&self, game: &Game) -> HandType {
        let mut score = game.hand().score();

        let joker_count = match self.1 {
            Some(joker) => {
                let entry = score.entry(joker).or_default();
                let joker_count = *entry;

                *entry = 0;
                joker_count
            }
            _ => 0,
        };

        let (first, second) = score
            .values()
            .cloned()
            .sorted()
            .rev()
            .take(2)
            .collect_tuple()
            .unwrap_or_default();

        match (first + joker_count, second) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_indexes(&self, game: &Game) -> Vec<usize> {
        game.hand()
            .iter()
            .filter_map(|card| self.index(card))
            .collect_vec()
    }

    fn play(&self, games: &[Game]) -> Option<usize> {
        games
            .iter()
            .map(|game| (self.get_type(game), self.get_indexes(game), game.bid()))
            .sorted()
            .zip(1..)
            .map(|((_, _, bid), rank)| rank * bid)
            .sum1()
    }
}

impl FromStr for Game {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or(SolutionError::ParseError)?;

        Ok(Game(
            Hand::new(hand),
            bid.parse().map_err(|_| SolutionError::ParseError)?,
        ))
    }
}

////////////////

struct Day07;

impl Solution for Day07 {
    const TITLE: &'static str = "Camel Cards";
    const DAY: u8 = 7;
    type Input = Vec<Game>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        input.lines().map(Game::from_str).collect()
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let camel_card = CamelCard::new("23456789TJQKA", None);

        camel_card.play(input)
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let camel_card = CamelCard::new("J23456789TQKA", Some('J'));

        camel_card.play(input)
    }
}

aoc::run!(Day07);

aoc::example! {
    [Day07]
    example: "32T3K 765\r\nT55J5 684\r\nKK677 28\r\nKTJJT 220\r\nQQQJA 483"
        => Some(6440)
        => Some(5905)
}
