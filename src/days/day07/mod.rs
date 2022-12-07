mod command;
mod file_system;

use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::identities::Zero;

use self::command::{Command, CommandFactory};
use self::file_system::{Context, NodeLike};
use crate::solver::{Error, PuzzlePart, Result, Solve};

pub struct Solver {
    commands: Vec<Box<dyn Command>>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let commands = reader.lines().fold(Ok::<_, Error>(Vec::default()), |commands, line| {
            let (mut commands, line) = (commands?, line?);

            if let Some(s) = line.strip_prefix('$') {
                commands.push(CommandFactory::parse_str(s)?);
            } else {
                commands
                    .last_mut()
                    .ok_or_else(|| Error::InvalidInput(format!("missing outputs' command: {line}")))?
                    .add_output(&line)?;
            }

            Ok(commands)
        })?;

        Ok(Self { commands })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let mut context = Context::default();

        context.update(&self.commands)?;

        match puzzle_part {
            PuzzlePart::One => {
                let small_directories =
                    context.browse_from_root(|node| node.is_directory() && node.size() < 100_000u64.into());

                (!small_directories.is_empty())
                    .then(|| {
                        small_directories
                            .into_iter()
                            .map(|node| node.size())
                            .sum::<BigUint>()
                            .to_string()
                    })
                    .ok_or_else(|| Error::NoSolution("no directory smaller than 100000".to_string()))
            }
            PuzzlePart::Two => {
                let update_space = 30_000_000u64.into();
                let total_space = 70_000_000u64.into();
                let used_space = context.root().size();

                if used_space > total_space {
                    return Err(Error::NoSolution(format!(
                        "used space overflow total disk space: {used_space} > 70000000"
                    )));
                }
                let free_space = total_space - used_space;

                if free_space > update_space {
                    return Err(Error::NoSolution(format!(
                        "file system already contains enough free space: {free_space}"
                    )));
                }

                let required_space = update_space - free_space;

                Ok(context
                    .browse_from_root(|node| node.is_directory() && node.size() >= required_space)
                    .into_iter()
                    .map(|node| node.size())
                    .sorted()
                    .take(1)
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap_or(&BigUint::zero())
                    .to_string())
            }
        }
    }
}
