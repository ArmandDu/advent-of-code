use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;

use aoc::solution::SolutionError;
use aoc::Solution;
use itertools::Itertools;

use aoc_utils::lcm;

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
    Named,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct CommandCenter(HashMap<String, (Module, Vec<String>)>);

impl Pulse {
    fn flip(&mut self) {
        *self = match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        };
    }
}

impl Module {
    fn pulse(&self) -> Option<Pulse> {
        match self {
            Module::FlipFlop(state) => Some(*state),
            Module::Conjunction(mem) => mem
                .values()
                .all(|pulse| pulse == &Pulse::High)
                .then_some(Pulse::Low)
                .or(Some(Pulse::High)),
            Module::Named => None,
        }
    }

    fn handle_event(&mut self, source: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop(state) => (pulse == Pulse::Low).then(|| {
                state.flip();
                *state
            }),
            Module::Conjunction(mem) => {
                *mem.entry(source.to_owned()).or_insert(Pulse::Low) = pulse;

                self.pulse()
            }
            Module::Named => Some(pulse),
        }
    }
}

impl CommandCenter {
    fn get_mut(&mut self, name: &str) -> Option<&mut (Module, Vec<String>)> {
        self.0.get_mut(name)
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &(Module, Vec<String>))> {
        self.0.iter()
    }

    fn push_button(&mut self, mut op: impl FnMut(&str, &str, Pulse) -> bool) {
        let mut queue = VecDeque::new();

        queue.push_back(("button".to_owned(), "broadcaster".to_owned(), Pulse::Low));

        while let Some((source, name, pulse)) = queue.pop_front() {
            if !op(&source, &name, pulse) {
                break;
            }

            if let Some((module, dest)) = self.get_mut(&name) {
                if let Some(next_pulse) = module.handle_event(&source, pulse) {
                    for sub in dest.clone() {
                        queue.push_back((name.to_owned(), sub, next_pulse));
                    }
                }
            }
        }
    }
}

impl FromStr for CommandCenter {
    type Err = SolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                let (name, nodes) = line.split_once(" -> ").ok_or(SolutionError::ParseError)?;

                let (name, module) = match &name[0..1] {
                    "%" => (&name[1..], Module::FlipFlop(Pulse::Low)),
                    "&" => (&name[1..], Module::Conjunction(HashMap::new())),
                    _ => (name, Module::Named),
                };

                let nodes = nodes
                    .split(',')
                    .map(|name| name.trim().to_owned())
                    .collect_vec();

                Ok((name.to_owned(), (module, nodes)))
            })
            .collect::<Result<_, _>>()
            .map(|mut modules: HashMap<String, (Module, Vec<String>)>| {
                let sources = modules
                    .iter()
                    .flat_map(|(origin, (_, subs))| {
                        subs.iter().map(|sub| (sub.to_owned(), origin.to_owned()))
                    })
                    .collect_vec();

                for (sub, source) in sources {
                    if let Some((Module::Conjunction(state), _)) = modules.get_mut(&sub) {
                        state.entry(source).or_insert(Pulse::Low);
                    }
                }

                modules
            })
            .map(CommandCenter)
    }
}

struct Day20;

impl Solution for Day20 {
    const TITLE: &'static str = "Pulse Propagation";
    const DAY: u8 = 20;
    type Input = CommandCenter;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        CommandCenter::from_str(input)
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        let mut command_center = input.to_owned();

        (0..1000)
            .fold(HashMap::new(), |mut acc, _| {
                let mut count = HashMap::new();

                command_center.push_button(|_, _, pulse| {
                    *count.entry(pulse).or_insert(0) += 1;
                    true
                });

                count.into_iter().for_each(|(pulse, count)| {
                    *acc.entry(pulse).or_insert(0) += count;
                });

                acc
            })
            .values()
            .product1()
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        let mut command_center = input.to_owned();

        let (last_module_source, (module, _)) = input
            .iter()
            .filter(|(_, (_, subs))| subs.contains(&"rx".to_owned()))
            .exactly_one()
            .ok()?;

        let mut target_modules = match module {
            Module::Conjunction(state) => Some(
                state
                    .keys()
                    .map(|name| (name.to_owned(), 0))
                    .collect::<HashMap<_, _>>(),
            ),
            _ => None,
        }?;

        (1..).find(|index| {
            command_center.push_button(|source, dest, pulse| {
                if dest == last_module_source && pulse == Pulse::High {
                    if let Some(o) = target_modules.get_mut(source) {
                        *o = *index;
                    }
                }
                //continue if...
                target_modules.values().any(|count| count == &0)
            });

            target_modules.values().all(|count| count > &0)
        });

        target_modules.values().cloned().reduce(lcm)
    }
}

aoc::run!(Day20);

aoc::example! {
    [Day20]
    constant: "broadcaster -> a, b, c\r\n%a -> b\r\n%b -> c\r\n%c -> inv\r\n&inv -> a\r\n"
        => Some(32000000)
        => None
    variable: "broadcaster -> a\r\n%a -> inv, con\r\n&inv -> b\r\n%b -> con\r\n&con -> output\r\n"
        => Some(11687500)
        => None
}
