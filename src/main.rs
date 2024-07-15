use std::{env, fs, io};

enum PuzzlePart {
    ONE,
    TWO
}

trait Solveable {
    fn solve(&self, part: &Puzzle) -> String;
}

struct Puzzle {
    day: i16,
    text: String,
}

impl Puzzle {
    pub fn from_puzzle_file(day: i16) -> Result<Self, io::Error> {
        let path = env::current_dir()?
            .join("static")
            .join("inputs")
            .join("day".to_string() + &day.to_string() + &".txt");

        let text = fs::read_to_string(path)?;
        Ok(Self { day, text })
    }
}

struct Day01 {
    puzzle: Puzzle
}

impl Solveable for Day01 {
    fn solve(&self, part: &Puzzle) -> String {
        String::from("puzzle1")
    }
}

struct Day02 {
    puzzle: Puzzle
}

impl Solveable for Day02 {
    fn solve(&self, part: &Puzzle) -> String {
        String::from("puzzle2")
    }
}

enum Day {
    One(Day01),
    Two(Day02)
}

type SolveResult = (Option<String>, Option<String>);
type Solver = fn(&Puzzle) -> SolveResult;

fn solve_day_one(puzzle: &Puzzle) -> (Option<String>, Option<String>) {
    (None, None)
}

fn solve_day_two(puzzle: &Puzzle) -> (Option<String>, Option<String>) {
    (None, None)
}

fn main() -> Result<(), io::Error>{

    // if no arguments are passed then run all

    // else only execute the days passed via the command line

    let solvers: Vec<Solver> = vec![
        solve_day_one,
        solve_day_two
    ];
    let day_range = 1..=2;

    for (day, solver) in day_range.zip(solvers) {
        let puzzle = Puzzle::from_puzzle_file(day)?;
        
        match solver(&puzzle) {
            (None, None) => println!("No puzzle solved for day {}!", day),
            (Some(v), None) => println!(""),
            (None, Some(v)) => println!(""),
            (Some(a), Some(b)) => println!("")
        }
    }

    Ok(())
}
