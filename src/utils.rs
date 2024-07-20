use std::fs;
use std::io;
use std::env;

pub type SolveResult = (Option<String>, Option<String>);
pub type Solver = fn(&Puzzle) -> SolveResult;

pub struct Puzzle {
    pub text: String,
}

impl Puzzle {
    pub fn from_puzzle_file(day: i16, example_file: bool) -> Result<Self, io::Error> {
        let stem = "day".to_string() + &day.to_string();
        let file_name = if example_file { stem + &".txt.example" } else { stem + &".txt" };
        let path = env::current_dir()?
            .join("static")
            .join("inputs")
            .join(file_name);

        let text = fs::read_to_string(path)?;
        Ok(Self { text })
    }
}
