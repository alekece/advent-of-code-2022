use std::io::{BufRead, BufReader, Read};

use eyre::Context;
use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Default)]
pub struct Elf {
    foods: Vec<usize>,
}

impl Elf {
    pub fn total_calories(&self) -> usize {
        self.foods.iter().sum()
    }

    pub fn add_food(&mut self, calories: usize) {
        self.foods.push(calories);
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

                    if line.is_empty() {
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

        if elves.is_empty() {
            Err(Error::EmptyInput)
        } else {
            Ok(Self { elves })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let calories = self.elves.iter().map(Elf::total_calories);

        let solution = match puzzle_part {
            PuzzlePart::One => calories.max().unwrap_or_default().to_string(),
            PuzzlePart::Two => calories.sorted().rev().take(3).sum::<usize>().to_string(),
        };

        Ok(solution)
    }
}
