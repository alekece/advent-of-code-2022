use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

use aoc_core::FromFile;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Invalid choice '{0}'")]
    InvalidChoice(String),
    #[error("Invalid round '{0}'")]
    InvalidRound(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    pub fn is_weakness_of(&self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper)
        )
    }

    pub fn get_weakness(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn get_resistance(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

impl FromStr for Choice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(Error::InvalidChoice(s.to_string())),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl FromStr for RoundResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lost),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Won),
            _ => Err(Error::InvalidRound(s.to_string())),
        }
    }
}

pub struct PossibleAction {
    round_result: RoundResult,
    choice: Choice,
}

impl FromStr for PossibleAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            round_result: s.parse::<RoundResult>()?,
            choice: s.parse::<Choice>()?,
        })
    }
}

impl From<(Choice, Choice)> for RoundResult {
    fn from(choices: (Choice, Choice)) -> Self {
        match choices {
            (a, b) if a.is_weakness_of(b) => RoundResult::Lost,
            (a, b) if b.is_weakness_of(a) => RoundResult::Won,
            _ => RoundResult::Draw,
        }
    }
}

pub struct GameSolver {
    rounds: Vec<(Choice, PossibleAction)>,
}

impl GameSolver {
    pub fn follow_elf_strategy(&self) -> u32 {
        self.rounds
            .iter()
            .map(|(a, b)| (*a, b.choice))
            .fold(0u32, |acc, (a, b)| acc + RoundResult::from((a, b)) as u32 + b as u32)
    }

    pub fn follow_fixed_elf_strategy(&self) -> u32 {
        self.rounds
            .iter()
            .map(|(a, b)| {
                {
                    let b = match b.round_result {
                        RoundResult::Won => a.get_weakness(),
                        RoundResult::Draw => *a,
                        RoundResult::Lost => a.get_resistance(),
                    };

                    (*a, b)
                }
            })
            .fold(0u32, |acc, (a, b)| acc + RoundResult::from((a, b)) as u32 + b as u32)
    }
}

impl FromFile for GameSolver {
    type Error = Error;

    fn from_file(path: &std::path::Path) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file).lines();

        let rounds = reader
            .into_iter()
            .map(|line| {
                let line = line?;

                line.split_once(' ')
                    .ok_or_else(|| Error::InvalidRound(line.clone()))
                    .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rounds })
    }
}
