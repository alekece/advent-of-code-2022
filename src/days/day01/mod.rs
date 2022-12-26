use std::io::{self, Read};

use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use pest::Parser;
use pest_derive::Parser;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Default)]
pub struct Elf {
    foods: Vec<BigUint>,
}

impl Elf {
    pub fn new(foods: Vec<BigUint>) -> Self {
        Self { foods }
    }

    pub fn total_calories(&self) -> BigUint {
        self.foods.iter().fold(BigUint::zero(), |acc, value| acc + value)
    }
}

#[derive(Parser)]
#[grammar = "days/day01/grammar.pest"]
pub struct Solver {
    elves: Vec<Elf>,
}

impl Solver {
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        let input = io::read_to_string(reader)?;

        let tokens = Self::parse(Rule::Input, &input)
            .map_err(|e| Error::InvalidInput(e.to_string()))?
            .next()
            .unwrap();

        let elves = tokens
            .into_inner()
            .filter_map(|token| {
                if let Rule::Elf = token.as_rule() {
                    Some(Elf::new(
                        token.into_inner().map(|v| v.as_str().parse().unwrap()).collect(),
                    ))
                } else {
                    None
                }
            })
            .collect();

        Ok(Self { elves })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let calories = self.elves.iter().map(Elf::total_calories).collect::<Vec<_>>();

        let solution = match puzzle_part {
            PuzzlePart::One => calories.into_iter().max().unwrap_or_default().to_string(),
            PuzzlePart::Two => calories.into_iter().sorted().rev().take(3).sum::<BigUint>().to_string(),
        };

        Ok(solution)
    }
}
