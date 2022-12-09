use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use eyre::Context;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use strum::{Display, EnumString};

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Copy, Clone, EnumString, Display)]
pub enum Direction {
    #[strum(serialize = "R")]
    Right,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "D")]
    Down,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: BigInt,
    y: BigInt,
}

impl Position {
    pub fn follow(&mut self, target: &Position) {
        let (x, y) = match (
            (target.x.clone() - self.x.clone()).try_into().unwrap(),
            (target.y.clone() - self.y.clone()).try_into().unwrap(),
        ) {
            (0, 0) | (1, 1) | (-1, -1) | (-1, 1) | (1, -1) | (1, 0) | (-1, 0) | (0, 1) | (0, -1) => (0, 0),
            (0, y) => (0, if y < 0 { y + 1 } else { y - 1 }),
            (x, 0) => (if x < 0 { x + 1 } else { x - 1 }, 0),
            (x, y) => {
                (
                    if x < 0 { (x + 1).min(-1) } else { (x - 1).max(1) },
                    if y < 0 { (y + 1).min(-1) } else { (y - 1).max(1) },
                )
            }
        };

        self.x += x;
        self.y += y;
    }

    pub fn advance(&mut self, direction: Direction) {
        let (x_direction, y_direction) = match direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };

        self.x += x_direction;
        self.y += y_direction;
    }
}

pub struct Movement {
    magnitude: BigInt,
    direction: Direction,
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_once(' ')
            .ok_or_else(|| {
                Error::InvalidInput(format!(
                    "wrong move: expected '{{direction}} {{magnitude}}' (got '{s}')"
                ))
            })
            .and_then(|(direction, magnitude)| {
                Ok(Self {
                    magnitude: magnitude.parse().wrap_err_with(|| {
                        Error::InvalidInput(format!("wrong magnitude: expected a valid integer (got '{magnitude}')"))
                    })?,
                    direction: direction.parse().wrap_err_with(|| {
                        Error::InvalidInput(format!("wrong direction: expected '{{R|L|U|D}}' (got '{direction}')"))
                    })?,
                })
            })
    }
}

pub struct Solver {
    movements: Vec<Movement>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let movements = reader.lines().map(|line| line?.parse()).collect::<Result<Vec<_>>>()?;

        Ok(Self { movements })
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        let size = match puzzle_part {
            PuzzlePart::One => 2,
            PuzzlePart::Two => 10,
        };

        let mut knots = vec![Position::default(); size];
        let mut registered_positions = [Position::default()].into_iter().collect::<HashSet<_>>();

        for movement in self.movements.iter() {
            let mut magnitude = BigInt::zero();

            // unfortunately `BigInt` does not implement `Step`
            while magnitude < movement.magnitude {
                knots[0].advance(movement.direction);

                let mut i = 0;

                while i < knots.len() - 1 {
                    let parent = knots[i].clone();
                    let child = &mut knots[i + 1];

                    child.follow(&parent);
                    i += 1;
                }

                registered_positions.insert(knots.last().unwrap().clone());
                magnitude += 1;
            }
        }

        Ok(registered_positions.len().to_string())
    }
}
