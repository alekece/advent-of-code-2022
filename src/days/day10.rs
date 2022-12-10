use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use enum_dispatch::enum_dispatch;
use eyre::Context;
use num_bigint::BigInt;
use num_traits::identities::Zero;

use crate::solver::{Error, PuzzlePart, Result, Solve};

pub enum Cycle {
    Wait,
    Done,
}

#[enum_dispatch(Command)]
pub trait CommandLike {
    fn execute(&mut self, register: &mut BigInt) -> Cycle;
}

#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Command {
    Noop(NoopCommand),
    AddX(AddXCommand),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("noop") {
            (s == "noop")
                .then_some(Self::Noop(NoopCommand))
                .ok_or_else(|| Error::InvalidInput(format!("wrong command: noop has no argument (got '{s}')")))
        } else if s.starts_with("addx") {
            s.split_once(' ')
                .ok_or_else(|| Error::InvalidInput(format!("wrong command: expected 'addx [0-9]+' (got '{s}')")))
                .and_then(|(_, value)| {
                    let value = value.parse().wrap_err_with(|| {
                        format!("wrong command: addx argument must be a valid integer (got '{value}')")
                    })?;

                    Ok(Self::AddX(AddXCommand::new(value)))
                })
        } else {
            Err(Error::InvalidInput(format!("unknown '{s}' command")))
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoopCommand;

impl CommandLike for NoopCommand {
    fn execute(&mut self, _register: &mut BigInt) -> Cycle {
        Cycle::Done
    }
}

#[derive(Clone, Debug)]
pub struct AddXCommand {
    value: BigInt,
    state: bool,
}

impl AddXCommand {
    pub fn new(value: BigInt) -> Self {
        Self { value, state: false }
    }
}

impl CommandLike for AddXCommand {
    fn execute(&mut self, register: &mut BigInt) -> Cycle {
        match self.state {
            true => {
                *register += self.value.clone();
                self.state = false;

                Cycle::Done
            }
            _ => {
                self.state = true;

                Cycle::Wait
            }
        }
    }
}

pub struct Solver {
    commands: Vec<Command>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let commands = reader.lines().map(|line| line?.parse()).collect::<Result<Vec<_>>>()?;

        Ok(Self { commands })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let mut commands = self.commands.iter().cloned().rev().collect::<Vec<_>>();
        let mut register = BigInt::from(1);

        match puzzle_part {
            PuzzlePart::One => {
                let mut command = commands.pop();
                let mut computed_value = BigInt::zero();
                let mut watched_cycle = 20;

                for n in 1..=220 {
                    if n == watched_cycle {
                        computed_value += register.clone() * watched_cycle;
                        watched_cycle += 40;
                    }

                    if let Cycle::Done = command
                        .as_mut()
                        .map(|command| command.execute(&mut register))
                        .unwrap_or(Cycle::Wait)
                    {
                        command = commands.pop();
                    }
                }

                Ok(computed_value.to_string())
            }
            PuzzlePart::Two => {
                let mut command = commands.pop();
                let mut result = String::default();

                for n in 0..240 {
                    let x = n % 40;

                    if register == x.into() || register == (x - 1).into() || register == (x + 1).into() {
                        result.push('#');
                    } else {
                        result.push('.');
                    }

                    if x == 39 {
                        result.push('\n');
                    }

                    if let Cycle::Done = command
                        .as_mut()
                        .map(|command| command.execute(&mut register))
                        .unwrap_or(Cycle::Wait)
                    {
                        command = commands.pop();
                    }
                }

                Ok(result)
            }
        }
    }
}
