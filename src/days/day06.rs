use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

use crate::solver::{Error, PuzzlePart, Result, Solve};

pub struct Solver {
    signal: Vec<u8>,
}

impl Solver {
    pub fn start_packet(&self, n: usize) -> Option<usize> {
        self
            .signal
            .windows(n)
            .enumerate()
            .find(|(_, sequence)| {
                let distinct_characters =  sequence.iter().collect::<HashSet<_>>();

                distinct_characters.len() == sequence.len()
            })
            // skips start packet bits
            .map(|(i, _)| i + n)
    }
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let mut signals = reader
            .lines()
            .map(|line| Ok(line?.chars().map(|c| c as u8).collect::<Vec<_>>()))
            .collect::<Result<Vec<_>>>()?;

        match signals.len() {
            0 => Err(Error::EmptyInput),
            1 => {
                Ok(Self {
                    signal: signals.pop().unwrap(),
                })
            }
            _ => {
                Err(Error::InvalidInput(format!(
                    "expected only one signal (got {} signals)",
                    signals.len()
                )))
            }
        }
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let n = match puzzle_part {
            PuzzlePart::One => 4,
            PuzzlePart::Two => 14,
        };

        self.start_packet(n)
            .ok_or_else(|| Error::NoSolution(format!("no consecutive '{n}' distinct characters in signal")))
            .map(|value| value.to_string())
    }
}
