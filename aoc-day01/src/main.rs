mod elf;

use std::path::PathBuf;

use aoc_core::{FromFile, PuzzlePart};
use clap::Parser;
use eyre::Result;

use elf::ElfGroup;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
    #[arg(short, long)]
    puzzle_part: PuzzlePart,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let elves = ElfGroup::from_file(args.file.as_path())?;

    let calories = match args.puzzle_part {
        PuzzlePart::One => {
            elves.find_carryiest_elf().map(|elf| elf.total_calories()).unwrap_or_default()
        }
        PuzzlePart::Two => {
            elves.find_top_n_carryiest_elves(3).iter().map(|elf| elf.total_calories()).sum()
        }
    };

    println!("{calories}");

    Ok(())
}
