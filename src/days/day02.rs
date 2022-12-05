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
            _ => Err(Error::ParsingError(format!("invalid choice '{s}'"))),
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
            _ => Err(Error::ParsingError(format!("invalid round '{s}'"))),
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

pub struct Solver {
    rounds: Vec<(Choice, PossibleAction)>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let rounds = reader
            .lines()
            .into_iter()
            .map(|line| {
                let line = line?;

                line.split_once(' ')
                    .ok_or_else(|| Error::ParsingError(format!("invalid input '{line}'")))
                    .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { rounds })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> String {
        match puzzle_part {
            PuzzlePart::One => {
                self.rounds
                    .iter()
                    .map(|(a, b)| (*a, b.choice))
                    .fold(0u32, compute_round)
                    .to_string()
            }
            PuzzlePart::Two => {
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
                    .fold(0u32, compute_round)
                    .to_string()
            }
        }
    }
}

fn compute_round(acc: u32, (a, b): (Choice, Choice)) -> u32 {
    acc + RoundResult::from((a, b)) as u32 + b as u32
}
