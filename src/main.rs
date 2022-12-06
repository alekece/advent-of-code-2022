use std::path::Path;

use advent_of_code_2022::solver::{PuzzlePart, Solve, Solver};
use clap::Parser;
use eyre::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    #[arg(short, long)]
    example: Option<Option<u16>>,
    #[arg(short, long)]
    puzzle_part: PuzzlePart,
    #[arg(short, long)]
    day: u16,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let opt = Opt::parse();

    let file_prefix = if let Some(example) = opt.example {
        format!("example{}", example.unwrap_or(1))
    } else {
        "input".to_string()
    };
    let file = format!("./data/{file_prefix}_day{:0width$}", opt.day, width = 2);

    let solver = Solver::from_file(Path::new(&file), opt.day)?;
    let solution = solver.solve(opt.puzzle_part)?;

    println!("{solution}");

    Ok(())
}
