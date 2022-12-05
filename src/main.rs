use std::path::Path;

use advent_of_code_2022::solver::{PuzzlePart, Solver, Solve};
use clap::Parser;
use eyre::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    #[arg(short, long)]
    example: bool,
    #[arg(short, long)]
    puzzle_part: PuzzlePart,
    #[arg(short, long)]
    day: u16,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();
    let file = format!(
        "./data/{}_day{:0width$}",
        if opt.example { "example" } else { "input" },
        opt.day,
        width = 2
    );
    let solver = Solver::from_file(Path::new(&file), opt.day)?;

    println!("{}", solver.solve(opt.puzzle_part));

    Ok(())
}
