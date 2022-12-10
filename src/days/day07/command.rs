use eyre::Context as _;
use num_bigint::BigUint;

use super::file_system::{Context, Node, NodeLike};
use crate::solver::{Error, Result};

pub trait Command: std::fmt::Debug {
    fn execute(&self, context: &mut Context) -> Result<()>;
    fn add_output(&mut self, output: &str) -> Result<()>;
}

pub struct CommandFactory;

impl CommandFactory {
    pub fn parse_str(s: &str) -> Result<Box<dyn Command>> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();

        match tokens.as_slice() {
            ["cd", target] => Ok(Box::new(ChangeDirectory::new(target.to_string()))),
            ["ls"] => Ok(Box::<ListDirectory>::default()),
            ["cd"] => {
                Err(Error::InvalidInput(
                    "wrong cd command: missing target directory".to_string(),
                ))
            }
            [command, ..] => Err(Error::InvalidInput(format!("unknown command '{command}'"))),
            [] => Err(Error::InvalidInput("missing command".to_string())),
        }
    }
}

#[derive(Debug)]
pub struct ChangeDirectory {
    target: String,
}

impl ChangeDirectory {
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

impl Command for ChangeDirectory {
    fn execute(&self, context: &mut Context) -> Result<()> {
        let target = self.target.as_str();

        if target == "/" {
            context.reset_to_root();
        } else {
            match target {
                ".." => {
                    let old_node = context
                        .move_back()
                        .ok_or_else(|| Error::NoSolution("cd ..: no working directory".to_string()))?;

                    if context.working_directory().is_none() {
                        return Err(Error::NoSolution(format!(
                            "cd ..: working directory '{}' has not parent",
                            old_node.name()
                        )));
                    }
                }
                name => {
                    let node = context
                        .working_directory()
                        .ok_or_else(|| Error::NoSolution(format!("cd {target}: no working directory")))?
                        .children()
                        .into_iter()
                        .find(|node| node.is_directory() && node.name() == name)
                        .ok_or_else(|| Error::NoSolution(format!("cd {name}: no such directory")))?;

                    context.go_to(node);
                }
            }
        }

        Ok(())
    }

    fn add_output(&mut self, _output: &str) -> Result<()> {
        Err(Error::InvalidInput(format!(
            "cd {}: command does not any output",
            self.target
        )))
    }
}

#[derive(Debug, Default)]
pub struct ListDirectory {
    outputs: Vec<(String, Option<BigUint>)>,
}

impl Command for ListDirectory {
    fn execute(&self, context: &mut Context) -> Result<()> {
        let mut current_node = context
            .working_directory()
            .ok_or_else(|| Error::NoSolution("ls: no working directory".to_string()))?;

        for (name, size) in self.outputs.iter() {
            let node = if let Some(size) = size {
                Node::new_file(name.clone(), size.clone())
            } else {
                Node::new_directory(name.clone())
            };

            current_node.add_child(node);
        }

        Ok(())
    }

    fn add_output(&mut self, output: &str) -> Result<()> {
        let output = match output.split_once(' ') {
            Some(("dir", name)) => Ok((name.to_string(), None)),
            Some((x, name)) => {
                Ok((
                    name.to_string(),
                    Some(x.parse::<BigUint>().wrap_err_with(|| {
                        Error::InvalidInput(format!(
                            "wrong ls output: file size must be a valid unsigned integer (got '{x}')"
                        ))
                    })?),
                ))
            }
            _ => {
                Err(Error::InvalidInput(format!(
                    "wrong ls output: expected '{{[0-9]+|dir}} {{name}}' (got '{output}')"
                )))
            }
        }?;

        self.outputs.push(output);

        Ok(())
    }
}
