use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Item(u8);

impl Item {
    pub fn get_priority(&self) -> usize {
        match self.0 as char {
            'a'..='z' => (self.0 - b'a' + 1) as usize,
            'A'..='Z' => (self.0 - b'A' + 27) as usize,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Item {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'a'..='z' | 'A'..='Z' => Ok(Self(c as u8)),
            _ => Err(Error::ParsingError(format!("invalid item '{c}'"))),
        }
    }
}

pub struct Rucksack {
    compartments: [HashSet<Item>; 2],
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() & 1 != 0 {
            return Err(Error::ParsingError(format!(
                "invalid rucksack '{s}': can only contains even list of items"
            )));
        }

        let half_len = s.len() / 2;

        let compartments = [
            s[0..half_len].chars().map(Item::try_from).collect::<Result<_>>()?,
            s[half_len..].chars().map(Item::try_from).collect::<Result<_>>()?,
        ];

        Ok(Self { compartments })
    }
}

impl Rucksack {
    pub fn find_misplaced_item(&self) -> Option<Item> {
        (&self.compartments[0] & &self.compartments[1]).into_iter().next()
    }

    pub fn get_all_items(&self) -> HashSet<Item> {
        self.compartments[0]
            .iter()
            .chain(self.compartments[1].iter())
            .copied()
            .collect()
    }
}

pub struct Solver {
    rucksacks: Vec<Rucksack>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let rucksacks = reader
            .lines()
            .into_iter()
            .map(|line| line?.parse())
            .collect::<Result<Vec<_>>>()?;

        if rucksacks.len() % 3 != 0 {
            Err(Error::ParsingError(
                "elf group must strictly contain 3 members".to_string(),
            ))
        } else {
            Ok(Self { rucksacks })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> String {
        match puzzle_part {
            PuzzlePart::One => {
                self.rucksacks
                    .iter()
                    .map(|r| r.find_misplaced_item().unwrap().get_priority())
                    .sum::<usize>()
                    .to_string()
            }
            PuzzlePart::Two => {
                self.rucksacks
                    .chunks_exact(3)
                    .flat_map(|x| &(&x[0].get_all_items() & &x[1].get_all_items()) & &x[2].get_all_items())
                    .map(|item| item.get_priority())
                    .sum::<usize>()
                    .to_string()
            }
        }
    }
}
