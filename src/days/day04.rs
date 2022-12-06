use std::io::{BufRead, BufReader, Read};
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Clone)]
pub struct Section(RangeInclusive<usize>);

impl Section {
    pub fn contains(&self, other: &Section) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }

    pub fn overlap(&self, other: &Section) -> bool {
        self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
}

impl FromStr for Section {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once('-') {
            Some((a, b)) => Ok(Self(a.parse()?..=b.parse()?)),
            None => {
                Err(Error::InvalidInput(format!(
                    "wrong section: expected '{{0-9}}+-{{0-9}}+' (got '{s}')"
                )))
            }
        }
    }
}

#[derive(Debug)]
pub struct PeerCleaning(Section, Section);

impl PeerCleaning {
    pub fn is_fully_overlapping(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn is_overlapping(&self) -> bool {
        self.0.overlap(&self.1) || self.1.overlap(&self.0)
    }
}

impl FromStr for PeerCleaning {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split_once(',') {
            Some((a, b)) => Ok(Self(a.parse()?, b.parse()?)),
            None => {
                Err(Error::InvalidInput(format!(
                    "invalid peer cleaning: expected '{{0-9}}+-{{0-9}},{{0-9}}+-{{0-9}}+' (got '{s}')"
                )))
            }
        }
    }
}

pub struct Solver {
    peer_cleanings: Vec<PeerCleaning>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let peer_cleanings = reader
            .lines()
            .into_iter()
            .map(|line| line?.parse())
            .collect::<Result<Vec<_>>>()?;

        if peer_cleanings.is_empty() {
            Err(Error::EmptyInput)
        } else {
            Ok(Self { peer_cleanings })
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        match puzzle_part {
            PuzzlePart::One => {
                Ok(self
                    .peer_cleanings
                    .iter()
                    .filter(|peer_cleaning| peer_cleaning.is_fully_overlapping())
                    .count()
                    .to_string())
            }
            PuzzlePart::Two => {
                Ok(self
                    .peer_cleanings
                    .iter()
                    .filter(|peer_cleaning| peer_cleaning.is_overlapping())
                    .count()
                    .to_string())
            }
        }
    }
}
