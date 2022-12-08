use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

use crate::solver::{Error, PuzzlePart, Result, Solve};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree(u8);

impl Tree {
    pub fn is_taller_than<'a>(&self, mut trees: impl Iterator<Item = &'a Self>) -> bool {
        trees.all(|tree| tree.0 < self.0)
    }

    pub fn distance_from<'a>(&self, trees: impl Iterator<Item = &'a Self>) -> usize {
        let (max_distance, _) = trees.size_hint();

        trees
            .enumerate()
            .find(|(_, tree)| tree.0 >= self.0)
            .map(|(distance, _)| distance + 1)
            .unwrap_or(max_distance)
    }
}

impl TryFrom<char> for Tree {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        c.is_ascii_digit()
            .then_some(Self(c as u8))
            .ok_or_else(|| Error::InvalidInput(format!("wrong tree: expected digit (got '{c}')")))
    }
}

pub struct Solver {
    forest: Vec<Vec<Tree>>,
}

impl Solver {
    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let forest = reader
            .lines()
            .map(|line| line?.chars().map(Tree::try_from).collect::<Result<Vec<_>>>())
            .collect::<Result<Vec<_>>>()?;

        if forest.is_empty() || forest.iter().all(|trees| trees.is_empty()) {
            Err(Error::EmptyInput)
        } else if !forest.iter().skip(1).all(|trees| trees.len() == forest[0].len()) {
            Err(Error::InvalidInput("trees are not planted in a grid".to_string()))
        } else {
            Ok(Self { forest })
        }
    }

    pub fn get_trees_from_cartesian_coordinates(&self, x: usize, y: usize) -> (Vec<Tree>, Vec<Tree>) {
        (
            self.forest[y].clone(),
            self.forest.iter().map(|trees| trees[x]).collect::<Vec<_>>(),
        )
    }

    pub fn compute_visible_trees(&self) -> usize {
        // calculate visible trees on edges.
        // note that trees in 2*X or X*2 grid are all visible.
        let count = if self.forest.len() < 2 || self.forest[0].len() < 2 {
            self.forest.len() * self.forest[0].len()
        } else {
            (self.forest.len() * 2) + (self.forest[0].len() - 2) * 2
        };

        count
            + (1..self.forest.len() - 1)
                .cartesian_product(1..self.forest[0].len() - 1)
                .filter(|(y, x)| {
                    let tree = self.forest[*y][*x];
                    let (horizontal_trees, vertical_trees) = self.get_trees_from_cartesian_coordinates(*x, *y);

                    tree.is_taller_than(horizontal_trees[..*x].iter())
                        || tree.is_taller_than(horizontal_trees[*x + 1..].iter())
                        || tree.is_taller_than(vertical_trees[..*y].iter())
                        || tree.is_taller_than(vertical_trees[*y + 1..].iter())
                })
                .count()
    }

    pub fn compute_highest_scenic_view(&self) -> usize {
        (1..self.forest.len() - 1)
            .cartesian_product(1..self.forest[0].len() - 1)
            .map(|(y, x)| {
                let tree = self.forest[y][x];
                let (horizontal_trees, vertical_trees) = self.get_trees_from_cartesian_coordinates(x, y);

                tree.distance_from(horizontal_trees[..x].iter().rev())
                    * tree.distance_from(horizontal_trees[x + 1..].iter())
                    * tree.distance_from(vertical_trees[..y].iter().rev())
                    * tree.distance_from(vertical_trees[y + 1..].iter())
            })
            .max()
            .unwrap_or_default()
    }
}

impl Solve for Solver {
    fn solve(&self, puzzle_part: PuzzlePart) -> Result<String> {
        match puzzle_part {
            PuzzlePart::One => Ok(self.compute_visible_trees().to_string()),
            PuzzlePart::Two => Ok(self.compute_highest_scenic_view().to_string()),
        }
    }
}
