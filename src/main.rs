use std::io;

use clap::Parser;

mod utils;
mod one;
mod two;
mod three;

use utils::{Puzzle, Solver};
use one::solve_day_one;
use two::solve_day_two;
use three::solve_day_three;



#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short('e'), long, default_value_t = false)]
    use_example: bool,

    #[arg(long, default_value_t = 1)]
    day_start: usize,

    #[arg(long, default_value_t = 3)]
    day_end: usize,
}

fn main() -> Result<(), io::Error>{
    let solvers: Vec<Solver> = vec![
        solve_day_one,
        solve_day_two,
        solve_day_three
    ];
    let args = Args::parse();

    let a = args.day_start.checked_sub(1).unwrap_or(1);
    let b = args.day_end.checked_sub(1).unwrap_or(1);
    for (solver, day) in &mut solvers[a..=b].iter().zip(args.day_start..=args.day_end) {
        let puzzle = Puzzle::from_puzzle_file(day as i16, args.use_example)?;

        match solver(&puzzle) {
            (None, None) => println!("No puzzle solved for day {}!", day),
            (Some(v), None) => println!("[{}] Part one: {}", day, v),
            (None, Some(v)) => println!("[{}] Part two: {}", day, v),
            (Some(a), Some(b)) => println!("[{}] Part one: {}\n[{}] Part two: {}", day, a, day, b)
        }
    }

    Ok(())
}
