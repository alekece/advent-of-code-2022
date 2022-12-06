use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

use crate::solver::{PuzzlePart, Result, Solve};

pub struct Solver {
    signals: Vec<u8>,
}

impl Solver {
    pub fn start_packet(&self, n: usize) -> usize {
        let (index, _) = self
            .signals
            .windows(n)
            .enumerate()
            .find(|(_, sequence)| sequence.iter().collect::<HashSet<_>>().len() == sequence.len())
            .unwrap();

        // skips start packet bits
        index + n
    }
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let signals = reader
            .lines()
            .take(1)
            .map(|line| Ok(line?.chars().map(|c| c as u8).collect::<Vec<_>>()))
            .collect::<Result<Vec<_>>>()?
            .pop()
            .unwrap_or_default();

        Ok(Self { signals })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> String {
        let n = match puzzle_part {
            PuzzlePart::One => 4,
            PuzzlePart::Two => 14,
        };

        self.start_packet(n).to_string()
    }
}
