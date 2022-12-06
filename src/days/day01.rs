use std::io::{BufRead, BufReader, Read};

use eyre::Context;
use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Default)]
pub struct Elf {
    foods: Vec<usize>,
}

impl Elf {
    pub fn total_calories(&self) -> Result<usize> {
        self.foods.iter().fold(Ok(0), |acc, b| {
            acc.and_then(|a| {
                a.checked_add(*b)
                    .ok_or_else(|| Error::NoSolution("cannot compute total calories: integer overflow".to_string()))
            })
        })
    }

    pub fn add_food(&mut self, calories: usize) {
        self.foods.push(calories);
    }

    pub fn has_foods(&self) -> bool {
        !self.foods.is_empty()
    }
}

pub struct Solver {
    elves: Vec<Elf>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let elves =
            reader
                .lines()
                .into_iter()
                .fold(Ok::<_, Error>(vec![Elf::default()]), |elves, line| {
                    let (mut elves, line) = (elves?, line?);

                    if line.is_empty() || elves.is_empty() {
                        elves.push(Elf::default());
                    } else {
                        // 'unwrap' call is safe since there is always at least
                        // one elf group in accumulator.
                        let elf = elves.last_mut().unwrap();

                        elf.add_food(line.parse().wrap_err_with(|| {
                            format!("calories must be a valid unsigned integer value (got '{line}')")
                        })?);
                    }

                    Ok(elves)
                })?;

        // `elves` vector will always contains at least one elf group then
        // having one elf with no food means the input was empty.
        if elves.len() == 1 && !elves[0].has_foods() {
            Err(Error::EmptyInput)
        } else {
            Ok(Self { elves })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let calories = self.elves.iter().map(Elf::total_calories).collect::<Result<Vec<_>>>()?;

        let solution = match puzzle_part {
            PuzzlePart::One => calories.into_iter().max().unwrap_or_default().to_string(),
            PuzzlePart::Two => calories.into_iter().sorted().rev().take(3).sum::<usize>().to_string(),
        };

        Ok(solution)
    }
}
