use std::path::Path;

use advent_of_code_2022::solver::{PuzzlePart, Solve, Solver};
use clap::{ArgGroup, Parser};
use eyre::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("input").args(["example", "file"])))]
pub struct Opt {
    /// Use user file as puzzle input
    #[arg(short, long)]
    file: Option<String>,
    /// Use puzzle examples as input
    #[arg(short, long)]
    example: Option<Option<u16>>,
    #[arg(short, long)]
    puzzle_part: PuzzlePart,
    /// Indicates the puzzle to solve by its referencing day
    #[arg(short, long)]
    day: u16,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();

    let file = opt.file.unwrap_or_else(|| {
        let prefix = if let Some(example) = opt.example {
            format!("example{}", example.unwrap_or(1))
        } else {
            "input".to_string()
        };

        format!("./data/{}_day{:0width$}", prefix, opt.day, width = 2)
    });

    let solver = Solver::from_file(Path::new(&file), opt.day)?;
    let solution = solver.solve(opt.puzzle_part)?;

    println!("{solution}");

    Ok(())
}
