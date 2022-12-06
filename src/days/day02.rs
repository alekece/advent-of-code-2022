use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use crate::solver::{Error, PuzzlePart, Result, Solve};

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

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => {
                Err(Error::InvalidInput(format!(
                    "wrong round choice: expected '{{A|B|C|X|Y|Z}}' (got '{s}')"
                )))
            }
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

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Self::Lost),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Won),
            _ => {
                Err(Error::InvalidInput(format!(
                    "wrong round result: expected '{{X|Y|Z}}' (got '{s}')"
                )))
            }
        }
    }
}

pub struct PossibleAction {
    round_result: RoundResult,
    choice: Choice,
}

impl FromStr for PossibleAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            choice: s.parse()?,
            round_result: s.parse()?,
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

pub struct Round(Choice, PossibleAction);

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_once(' ')
            .ok_or_else(|| Error::InvalidInput(format!("wrong round: expected '{{A|B|C}} {{X|Y|Z}}' (got '{s}'")))
            .and_then(|(a, b)| Ok(Self(a.parse()?, b.parse()?)))
    }
}

pub struct Solver {
    rounds: Vec<Round>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let rounds = reader
            .lines()
            .into_iter()
            .map(|line| line?.parse())
            .collect::<Result<Vec<_>>>()?;

        if rounds.is_empty() {
            Err(Error::EmptyInput)
        } else {
            Ok(Self { rounds })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        match puzzle_part {
            PuzzlePart::One => {
                Ok(self
                    .rounds
                    .iter()
                    .map(|round| (round.0, round.1.choice))
                    .fold(0u32, compute_round)
                    .to_string())
            }
            PuzzlePart::Two => {
                Ok(self
                    .rounds
                    .iter()
                    .map(|round| {
                        {
                            let choice = match round.1.round_result {
                                RoundResult::Won => round.0.get_weakness(),
                                RoundResult::Draw => round.0,
                                RoundResult::Lost => round.0.get_resistance(),
                            };

                            (round.0, choice)
                        }
                    })
                    .fold(0u32, compute_round)
                    .to_string())
            }
        }
    }
}

fn compute_round(acc: u32, (a, b): (Choice, Choice)) -> u32 {
    acc + RoundResult::from((a, b)) as u32 + b as u32
}
