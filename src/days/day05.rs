use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use eyre::Context;
use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

pub trait CrateMover {
    fn grab_crates(&self, stack: &mut VecDeque<char>, quantity: usize) -> VecDeque<char>;
}

pub struct CrateMover9000;

impl CrateMover for CrateMover9000 {
    fn grab_crates(&self, stack: &mut VecDeque<char>, quantity: usize) -> VecDeque<char> {
        stack.drain(stack.len() - quantity..).rev().collect()
    }
}

pub struct CrateMover9001;

impl CrateMover for CrateMover9001 {
    fn grab_crates(&self, stack: &mut VecDeque<char>, quantity: usize) -> VecDeque<char> {
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
    stacks: HashMap<usize, VecDeque<char>>,
    indices: HashMap<usize, usize>,
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
                is_stack &= !line.is_empty();

                is_stack
            });

        if stacks.is_empty() && instructions.is_empty() {
            return Err(Error::EmptyInput);
        }

        let splitter = |step_by: usize| {
            let mut counter = 0;

            move |_| {
                counter += 1;

                counter % step_by == 0
            }
        };

        let indices = stacks[stacks.len() - 1]
            .split(splitter(4))
            .enumerate()
            .map(|(i, s)| {
                match s.as_bytes() {
                    [b' ', key @ b'0'..=b'9', b' '] => Ok(((*key - b'0') as usize, i)),
                    [b' ', key @ b'0'..=b'9'] => Ok(((*key - b'0') as usize, i)),
                    _ => {
                        Err(Error::InvalidInput(format!(
                            "wrong indice: expected ' [0-9] ' (got '{s}')"
                        )))
                    }
                }
            })
            .collect::<Result<HashMap<usize, usize>>>()?;

        let stacks = stacks[..stacks.len() - 1]
            .iter()
            .flat_map(|line| {
                line.split(splitter(4))
                    .enumerate()
                    .filter_map(|(i, s)| {
                        match s.as_bytes() {
                            [b'[', c, b']'] => Some((i, Some(*c as char))),
                            [b' ', b' ', b' '] => Some((i, None)),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .fold(Ok(HashMap::<usize, VecDeque<char>>::default()), |stacks, (key, c)| {
                let mut stacks = stacks?;
                let stack = stacks.entry(key).or_default();

                if let Some(c) = c {
                    stack.push_front(c);
                } else if !stack.is_empty() {
                    return Err(Error::InvalidInput(
                        "wrong stack: floating crate(s) detected".to_string(),
                    ));
                }

                Ok(stacks)
            })?;

        let instructions = instructions
            .into_iter()
            // skips empty separator line
            .filter(|line| !line.is_empty())
            .map(|line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            stacks,
            instructions,
            indices,
        })
    }

    pub fn move_crates(&self, mover: impl CrateMover) -> Result<HashMap<usize, VecDeque<char>>> {
        let mut stacks = self.stacks.clone();

        for instruction in self.instructions.iter() {
            let stack = self
                .indices
                .get(&instruction.from_stack)
                .and_then(|i| stacks.get_mut(i))
                .ok_or_else(|| {
                    Error::NoSolution(format!("source stack '{}' does not exist", instruction.from_stack))
                })?;

            if stack.len() < instruction.quantity {
                return Err(Error::NoSolution(format!(
                    "cannot move {} crate(s) from stack '{}': stack contains only {} crate(s)",
                    instruction.quantity,
                    instruction.from_stack,
                    stack.len()
                )));
            }

            let mut crates = mover.grab_crates(stack, instruction.quantity);

            let stack = self
                .indices
                .get(&instruction.to_stack)
                .and_then(|i| stacks.get_mut(i))
                .ok_or_else(|| {
                    Error::NoSolution(format!("destination stack '{}' does not exist", instruction.to_stack))
                })?;

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
            .filter_map(|(_, crates)| crates.back().copied())
            .collect())
    }
}
