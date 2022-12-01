use std::ops::Deref;

use clap::{Args, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PuzzlePart {
    One,
    Two,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt<T: Args> {
    #[arg(short, long, value_enum)]
    pub puzzle_part: PuzzlePart,
    #[clap(flatten)]
    inner: T,
}

impl<T: Args> Deref for Opt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
