use std::io::{self, Read};

use pest::Parser;
use pest_derive::Parser;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

pub struct Action {
    play: Play,
    round_result: RoundResult,
}

impl Action {
    pub fn new(play: Play, round_result: RoundResult) -> Self {
        Self { play, round_result }
    }
}

pub struct Round(Play, Action);

#[derive(Parser)]
#[grammar = "days/day02/grammar.pest"]
pub struct Solver {
    rounds: Vec<Round>,
}

impl Solver {
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        let input = io::read_to_string(reader)?;

        let tokens = Self::parse(Rule::Input, &input)
            .map_err(|e| Error::InvalidInput(e.to_string()))?
            .next()
            .unwrap();

        let rounds = tokens
            .into_inner()
            .filter_map(|token| {
                if let Rule::Round = token.as_rule() {
                    let mut token = token.into_inner();

                    Some(Round(
                        match token.next().unwrap().as_str() {
                            "A" => Play::Rock,
                            "B" => Play::Paper,
                            "C" => Play::Scissors,
                            _ => unreachable!(),
                        },
                        match token.next().unwrap().as_str() {
                            "X" => Action::new(Play::Rock, RoundResult::Lost),
                            "Y" => Action::new(Play::Paper, RoundResult::Draw),
                            "Z" => Action::new(Play::Scissors, RoundResult::Won),
                            _ => unreachable!(),
                        },
                    ))
                } else {
                    None
                }
            })
            .collect();

        Ok(Self { rounds })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        match puzzle_part {
            PuzzlePart::One => {
                Ok(self
                    .rounds
                    .iter()
                    .map(|round| (round.0, round.1.play))
                    .fold(0u32, compute_round)
                    .to_string())
            }
            PuzzlePart::Two => {
                Ok(self
                    .rounds
                    .iter()
                    .map(|round| {
                        let play = match round.1.round_result {
                            RoundResult::Lost => round.0.get_resistance(),
                            RoundResult::Draw => round.0,
                            RoundResult::Won => round.0.get_weakness(),
                        };

                        (round.0, play)
                    })
                    .fold(0u32, compute_round)
                    .to_string())
            }
        }
    }
}

fn compute_round(acc: u32, (a, b): (Play, Play)) -> u32 {
    let round_result = if a.is_weakness_of(b) {
        RoundResult::Lost
    } else if b.is_weakness_of(a) {
        RoundResult::Won
    } else {
        RoundResult::Draw
    };

    acc + round_result as u32 + b as u32
}
