mod game;

use std::path::PathBuf;

use aoc_core::{FromFile, Opt, PuzzlePart};
use clap::{Args, Parser};
use eyre::Result;

use game::GameSolver;

#[derive(Args)]
struct CustomOpt {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::<CustomOpt>::parse();
    let solver = GameSolver::from_file(opt.file.as_path())?;

    let score = match opt.puzzle_part {
        PuzzlePart::One => solver.follow_elf_strategy(),
        PuzzlePart::Two => solver.follow_fixed_elf_strategy(),
    };

    println!("{score}");

    Ok(())
}
