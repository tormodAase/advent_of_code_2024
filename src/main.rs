mod day_one;
mod helpers;
mod solvers;

use clap::Parser;
use solvers::get_solver;
use solvers::get_solvers;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Day
    #[arg(short, long, default_value_t = 1)]
    day: usize,
    /// Puzzle. Can only be 1 or 2
    #[arg(short, long, default_value_t = 1)]
    puzzle: usize,
    /// Input string for the puzzle
    #[arg(short, long, default_value_t = String::from(""))]
    input: String,
}

fn main() {
    let args = Args::parse();
    let mut solvers = get_solvers();
    let maybe_puzzle = get_solver(&mut solvers, args.day, args.puzzle);
    match maybe_puzzle {
        None => println!("Puzzle not found. Make sure the day and puzzle number is correct. Use '--help' or '-h' for more information"),
        Some(mut puzzle) => {
            puzzle.parse(&args.input);
            println!(
                "Solution for day {}, puzzle {}:\n{}",
                args.day,
                args.puzzle,
                puzzle.solve()
            );
        }
    }
}
