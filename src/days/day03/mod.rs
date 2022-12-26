use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};

use pest::Parser;
use pest_derive::Parser;

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

pub struct Rucksack {
    items: Vec<Item>,
    compartments: [HashSet<Item>; 2],
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

#[derive(Parser)]
#[grammar = "days/day03/grammar.pest"]
pub struct Solver {
    rucksacks: Vec<Rucksack>,
}

impl Solver {
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        let input = io::read_to_string(reader)?;

        let tokens = Self::parse(Rule::Input, &input)
            .map_err(|e| Error::InvalidInput(e.to_string()))?
            .next()
            .unwrap();

        let rucksacks = tokens
            .into_inner()
            .filter_map(|token| {
                if let Rule::Rucksack = token.as_rule() {
                    let items = token
                        .into_inner()
                        .map(|v| Item(v.as_str().chars().next().unwrap() as u8))
                        .collect::<Vec<Item>>();

                    let pivot = items.len() / 2;
                    let compartments = [
                        items[0..pivot].iter().copied().collect(),
                        items[pivot..].iter().copied().collect(),
                    ];

                    Some(Rucksack { items, compartments })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if rucksacks.len() % 3 != 0 {
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
