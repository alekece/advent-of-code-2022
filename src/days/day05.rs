use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use eyre::Context;
use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

pub trait CrateMover {
    fn grab_crates(&self, stack: &mut Vec<char>, quantity: usize) -> Vec<char>;
}

pub struct CrateMover9000;

impl CrateMover for CrateMover9000 {
    fn grab_crates(&self, stack: &mut Vec<char>, quantity: usize) -> Vec<char> {
        stack.drain(stack.len() - quantity..).rev().collect()
    }
}

pub struct CrateMover9001;

impl CrateMover for CrateMover9001 {
    fn grab_crates(&self, stack: &mut Vec<char>, quantity: usize) -> Vec<char> {
        stack.drain(stack.len() - quantity..).collect()
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

        let values = s.split_whitespace().collect::<Vec<_>>();

        if values.len() != 6 {
            return Err(Error::InvalidInput(format!(
                "wrong instruction: expected 'move {{0-9}}+ from {{0-9}}+ to {{0-9}}+' (got '{s}')"
            )));
        }

        let values = values
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| {
                match TOKENS[i] {
                    Some(token) if token != s => {
                        Some(Err(Error::InvalidInput(format!(
                            "wrong instruction: expected token '{token}' (got '{s}')"
                        ))))
                    }
                    None => Some(s.parse::<usize>()
                                    .wrap_err_with(||
                                       Error::InvalidInput(format!("wrong instruction: quantity or stack indice must be a valid unsigned integer value (got '{s}')"))
                                    )
                                    .map_err(Into::into)),
                    _ => None,
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            quantity: values[0],
            from_stack: values[1],
            to_stack: values[2],
        })
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

        if stacks.is_empty() && instructions.is_empty() {
            return Err(Error::EmptyInput);
        }

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
            .filter(|line| !line.is_empty())
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { stacks, instructions })
    }

    pub fn move_crates(&self, mover: impl CrateMover) -> Result<HashMap<usize, Vec<char>>> {
        let mut stacks = self.stacks.clone();

        for instruction in self.instructions.iter() {
            let stack = stacks
                .get_mut(&instruction.from_stack)
                .ok_or_else(|| Error::NoSolution(format!("source stack '{}' does not exist", instruction.from_stack)))?;

            if stack.len() < instruction.quantity {
                return Err(Error::NoSolution(format!(
                    "cannot move {} crate(s) from stack '{}': stack contains only {} crate(s)",
                    instruction.quantity,
                    instruction.from_stack,
                    stack.len()
                )));
            }

            let mut crates = mover.grab_crates(stack, instruction.quantity);

            let stack = stacks
                .get_mut(&instruction.to_stack)
                .ok_or_else(|| Error::NoSolution(format!("destination stack '{}' does not exist", instruction.to_stack)))?;

            stack.append(&mut crates);
        }

        Ok(stacks)
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let stacks = match puzzle_part {
            PuzzlePart::One => self.move_crates(CrateMover9000),
            PuzzlePart::Two => self.move_crates(CrateMover9001),
        }?;

        Ok(stacks
            .into_iter()
            .sorted_by_key(|(key, _)| *key)
            .filter_map(|(_, crates)| crates.last().copied())
            .collect())
    }
}
