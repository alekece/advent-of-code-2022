mod elf;

use std::path::PathBuf;

use aoc_core::{FromFile, Opt, PuzzlePart};
use clap::{Args, Parser};
use eyre::Result;

use elf::ElfGroup;

#[derive(Args)]
struct CustomOpt {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::<CustomOpt>::parse();
    let elves = ElfGroup::from_file(opt.file.as_path())?;

    let calories = match opt.puzzle_part {
        PuzzlePart::One => {
            elves
                .find_carryiest_elf()
                .map(|elf| elf.total_calories())
                .unwrap_or_default()
        }
        PuzzlePart::Two => {
            elves
                .find_top_n_carryiest_elves(3)
                .iter()
                .map(|elf| elf.total_calories())
                .sum()
        }
    };

    println!("{calories}");

    Ok(())
}
