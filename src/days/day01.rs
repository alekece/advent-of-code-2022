use std::io::{BufRead, BufReader, Read};

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
        let elves = reader
            .lines()
            .into_iter()
            .fold(Ok::<_, Error>(vec![Elf::default()]), |elves, line| {
                let (mut elves, line) = (elves?, line?);

                if line.is_empty() {
                    elves.push(Elf::default());
                } else {
                    let elf = elves.last_mut().unwrap();
                    elf.add_food(line.parse()?);
                }

                Ok(elves)
            })?;

        Ok(Self { elves })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> String {
        let calories = self.elves.iter().map(Elf::total_calories);

        match puzzle_part {
            PuzzlePart::One => calories.max().unwrap_or_default().to_string(),
            PuzzlePart::Two => calories.sorted().rev().take(3).sum::<usize>().to_string(),
        }
    }
}
