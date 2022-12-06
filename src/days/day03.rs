use std::collections::HashSet;
use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use eyre::Context;

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
            _ => {
                Err(Error::InvalidInput(format!(
                    "wrong item: expected '{{a-z|A-Z}}' (got '{c}')"
                )))
            }
        }
    }
}

pub struct Rucksack {
    items: Vec<Item>,
    compartments: [HashSet<Item>; 2],
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() & 1 != 0 {
            return Err(Error::InvalidInput(format!(
                "wrong rucksack: expected list with even number of items (got '{s}')"
            )));
        }

        let half_len = s.len() / 2;
        let items = s
            .chars()
            .map(|c| {
                Ok(Item::try_from(c)
                    .wrap_err_with(|| Error::InvalidInput(format!("rucksack '{s}' contains invalid items")))?)
            })
            .collect::<Result<Vec<_>>>()?;
        let compartments = [
            items[0..half_len].iter().copied().collect(),
            items[half_len..].iter().copied().collect(),
        ];

        Ok(Self { items, compartments })
    }
}

impl fmt::Display for Rucksack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.items.iter() {
            write!(f, "{}", item.0 as char)?;
        }

        Ok(())
    }
}

impl Rucksack {
    pub fn find_misplaced_item(&self) -> Option<Item> {
        (&self.compartments[0] & &self.compartments[1]).into_iter().next()
    }

    pub fn get_all_items(&self) -> HashSet<Item> {
        self.items.iter().copied().collect()
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

        if rucksacks.is_empty() {
            Err(Error::EmptyInput)
        } else if rucksacks.len() % 3 != 0 {
            Err(Error::InvalidInput(
                "each elf group must strictly contain 3 members".to_string(),
            ))
        } else {
            Ok(Self { rucksacks })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        match puzzle_part {
            PuzzlePart::One => {
                let items = self
                    .rucksacks
                    .iter()
                    .map(|r| {
                        r.find_misplaced_item()
                            .ok_or_else(|| Error::NoSolution(format!("none misplaced item found in '{r}'")))
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(items
                    .into_iter()
                    .map(|item| item.get_priority())
                    .sum::<usize>()
                    .to_string())
            }
            PuzzlePart::Two => {
                Ok(self
                    .rucksacks
                    .chunks_exact(3)
                    .flat_map(|x| &(&x[0].get_all_items() & &x[1].get_all_items()) & &x[2].get_all_items())
                    .map(|item| item.get_priority())
                    .sum::<usize>()
                    .to_string())
            }
        }
    }
}
