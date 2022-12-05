use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

pub struct CrateMover9000;

impl CrateMover9000 {
    pub fn move_crates(
        mut stacks: HashMap<usize, Vec<char>>,
        instructions: &[Instruction],
    ) -> HashMap<usize, Vec<char>> {
        for instruction in instructions.iter() {
            let stack = stacks.get_mut(&instruction.from_stack).unwrap();
            let mut crates = stack
                .drain(stack.len() - instruction.quantity..)
                .rev()
                .collect::<Vec<_>>();

            let stack = stacks.get_mut(&instruction.to_stack).unwrap();
            stack.append(&mut crates);
        }

        stacks
    }
}

pub struct CrateMover9001;

impl CrateMover9001 {
    pub fn move_crates(
        mut stacks: HashMap<usize, Vec<char>>,
        instructions: &[Instruction],
    ) -> HashMap<usize, Vec<char>> {
        for instruction in instructions.iter() {
            let stack = stacks.get_mut(&instruction.from_stack).unwrap();
            let mut crates = stack.drain(stack.len() - instruction.quantity..).collect::<Vec<_>>();

            let stack = stacks.get_mut(&instruction.to_stack).unwrap();
            stack.append(&mut crates);
        }

        stacks
    }
}

#[derive(Debug)]
pub struct Instruction {
    quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        const TOKENS: &[Option<&'static str>] = &[Some("move"), None, Some("from"), None, Some("to"), None];

        let values = s
            .split_whitespace()
            .enumerate()
            .filter_map(|(i, s)| {
                match TOKENS[i] {
                    Some(token) if token != s => {
                        Some(Err(Error::ParsingError(format!(
                            "invalid instruction: expected token '{token}' (got '{s}')"
                        ))))
                    }
                    None => Some(s.parse::<usize>().map_err(Into::into)),
                    _ => None,
                }
            })
            .collect::<Result<Vec<_>>>()?;

        if values.len() != 3 {
            Err(Error::ParsingError(
                "invalid instruction (expected format 'move x from y to z')".to_string(),
            ))
        } else {
            Ok(Self {
                quantity: values[0],
                from_stack: values[1],
                to_stack: values[2],
            })
        }
    }
}

pub struct Solver {
    stacks: HashMap<usize, Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let mut is_stack = true;

        let (stacks, instructions): (Vec<_>, Vec<_>) = reader
            .lines()
            .map(|line| Ok(line?))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .partition(|line| {
                if line.is_empty() {
                    is_stack = false;
                }

                is_stack
            });

        let stacks = stacks
            .into_iter()
            .flat_map(|line| {
                let mut counter = 0;

                line.split(|_| {
                    counter += 1;

                    counter % 4 == 0
                })
                .enumerate()
                .filter_map(|(key, s)| {
                    match s.as_bytes() {
                        [b'[', c, b']'] => Some((key, *c as char)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
            })
            .rev()
            .fold(HashMap::<usize, Vec<char>>::default(), |mut acc, (key, c)| {
                acc.entry(key + 1).or_default().push(c);

                acc
            });

        let instructions = instructions
            .into_iter()
            // skips empty separator line
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { stacks, instructions })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> String {
        let stacks = match puzzle_part {
            PuzzlePart::One => CrateMover9000::move_crates(self.stacks.clone(), &self.instructions),
            PuzzlePart::Two => CrateMover9001::move_crates(self.stacks.clone(), &self.instructions),
        };

        stacks
            .into_iter()
            .sorted_by_key(|(key, _)| *key)
            .map(|(_, crates)| *crates.last().unwrap())
            .collect()
    }
}
