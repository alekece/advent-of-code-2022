use std::fs::File;
use std::io::{self, BufReader};
use std::num;
use std::path::Path;

use clap::ValueEnum;
use enum_dispatch::enum_dispatch;
use eyre::Context;
use thiserror::Error;

use crate::days::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    EyreReport(#[from] eyre::ErrReport),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ParseIntError(#[from] num::ParseIntError),
    #[error(transparent)]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
    #[error("Unimplemented day {0}")]
    UnimplementedDay(u16),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("No solution found: {0}")]
    NoSolution(String),
    #[error("Empty input")]
    EmptyInput,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PuzzlePart {
    One,
    Two,
}

#[enum_dispatch]
pub enum Solver {
    Day01(day01::Solver),
    Day02(day02::Solver),
    Day03(day03::Solver),
    Day04(day04::Solver),
    Day05(day05::Solver),
    Day06(day06::Solver),
    Day07(day07::Solver),
    Day08(day08::Solver),
}

#[enum_dispatch(Solver)]
pub trait Solve {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String>;
}

impl Solver {
    pub fn from_file(path: &Path, day: u16) -> Result<Self> {
        let file = File::open(path).wrap_err_with(|| format!("Cannot open file '{}'", path.display()))?;
        let reader = BufReader::new(file);

        let solver = match day {
            1 => day01::Solver::from_reader(reader)?.into(),
            2 => day02::Solver::from_reader(reader)?.into(),
            3 => day03::Solver::from_reader(reader)?.into(),
            4 => day04::Solver::from_reader(reader)?.into(),
            5 => day05::Solver::from_reader(reader)?.into(),
            6 => day06::Solver::from_reader(reader)?.into(),
            7 => day07::Solver::from_reader(reader)?.into(),
            8 => day08::Solver::from_reader(reader)?.into(),
            _ => return Err(Error::UnimplementedDay(day)),
        };

        Ok(solver)
    }
}
