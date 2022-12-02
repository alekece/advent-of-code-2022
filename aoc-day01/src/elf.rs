use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num;
use std::path::Path;

use aoc_core::FromFile;
use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ParseError(#[from] num::ParseIntError),
}

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

pub struct ElfGroup {
    elves: Vec<Elf>,
}

impl ElfGroup {
    pub fn find_carryiest_elf(&self) -> Option<&Elf> {
        self.elves.iter().max_by_key(|elf| elf.total_calories())
    }

    pub fn find_top_n_carryiest_elves(&self, n: usize) -> Vec<&Elf> {
        self.elves
            .iter()
            .sorted_by_key(|elf| elf.total_calories())
            .rev()
            .take(n)
            .collect()
    }
}

impl FromFile for ElfGroup {
    type Error = Error;

    fn from_file(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file).lines();

        let elves = reader
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
