use aoc::Solution;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Day07;

#[derive(Debug, Eq, PartialEq)]
pub enum Entry {
    Leave,
    Enter(String),
    Dir(String),
    File(String, usize),
    List,
}

impl TryFrom<&str> for Entry {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        const LS: &str = "$ ls";
        const CD: &str = "$ cd ";
        const PARENT: &str = "$ cd ..";
        const DIR: &str = "dir ";

        if line == LS {
            Ok(Self::List)
        } else if line == PARENT {
            Ok(Self::Leave)
        } else if line.starts_with(CD) {
            Ok(Self::Enter(line.strip_prefix(CD).ok_or(())?.to_owned()))
        } else if line.starts_with(DIR) {
            Ok(Self::Dir(line.strip_prefix(DIR).ok_or(())?.to_owned()))
        } else {
            let (size, name) = line.split_once(' ').ok_or(())?;

            Ok(Self::File(name.to_owned(), size.parse().map_err(|_| ())?))
        }
    }
}

impl Solution for Day07 {
    const TITLE: &'static str = "No Space Left On Device";
    const DAY: u8 = 7;
    type Input = HashMap<Vec<String>, usize>;
    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> aoc::solution::Result<Self::Input> {
        let fs = input
            .lines()
            .filter_map(|line| line.try_into().ok())
            .fold((HashMap::new(), vec![]), |(mut fs, mut path), entry| {
                match entry {
                    Entry::Leave => {
                        path.pop();
                    }
                    Entry::Enter(name) => {
                        path.push(name);
                        fs.insert(path.clone(), 0);
                    }
                    Entry::Dir(_) => {}
                    Entry::File(_, size) => {
                        let entry: &mut usize = fs.entry(path.clone()).or_default();

                        *entry += size;
                    }
                    Entry::List => {}
                };

                (fs, path)
            })
            .0;

        Ok(fs
            .iter()
            .map(|(path, _)| {
                let total_size = fs
                    .iter()
                    .filter(|(sub_path, _)| {
                        (0..path.len()).all(|index| path.get(index) == sub_path.get(index))
                    })
                    .map(|(_, size)| size)
                    .sum::<usize>();

                (path.clone(), total_size)
            })
            .collect())
    }

    fn part1(input: &Self::Input) -> Option<Self::P1> {
        Some(
            input
                .iter()
                .filter_map(|(_, size)| match size {
                    size if size <= &100000 => Some(size),
                    _ => None,
                })
                .sum::<usize>(),
        )
    }

    fn part2(input: &Self::Input) -> Option<Self::P2> {
        const FS_SIZE: usize = 70000000;
        const UPDATE_SIZE: usize = 30000000;

        let total_size = input.get(&vec!["/".to_owned()])?;
        let target = UPDATE_SIZE - (FS_SIZE - total_size);

        input
            .iter()
            .filter_map(|(_, size)| match size.cmp(&target) {
                Ordering::Less => None,
                _ => Some(size),
            })
            .min()
            .cloned()
    }
}

fn main() {
    aoc::solution!(Day07)
}
#[cfg(test)]
mod tests {
    use crate::Day07 as day_07;
    use crate::*;

    aoc::test_common!(day_07);

    aoc::test! {
        day_07:
        - "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n"
            => Some(95437)
            => Some(24933642)
    }

    #[test]
    fn str_to_entry() {
        assert_eq!("$ ls".try_into(), Ok(Entry::List));
        assert_eq!("$ cd /".try_into(), Ok(Entry::Enter("/".to_owned())));
        assert_eq!(
            "$ cd foobar".try_into(),
            Ok(Entry::Enter("foobar".to_owned()))
        );
        assert_eq!("$ cd ..".try_into(), Ok(Entry::Leave));
        assert_eq!("dir foobar".try_into(), Ok(Entry::Dir("foobar".to_owned())));
        assert_eq!(
            "4242 foobar.txt".try_into(),
            Ok(Entry::File("foobar.txt".to_owned(), 4242))
        );
    }
}
