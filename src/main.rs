use std::fs;
use std::env;

struct PuzzleInput {
    // load input from file system
    puzzle_input: String
}

impl PuzzleInput {
    fn new(day: u32) -> Self {
        let current_dir = env::current_dir().unwrap();
        let file_name = "day".to_owned() + &day.to_string() + &".txt";
        let file_path = "static/inputs/".to_owned() + &file_name;

        let file_path = current_dir.join(file_path);
        println!("Trying to read file from {}.", file_path.display());
        let puzzle_input = fs::read_to_string(file_path).expect("Unable to get read puzzle input file!");

        PuzzleInput { puzzle_input }
    }
}

enum PuzzleType {
    PartOne,
    PartTwo,
}

trait Puzzle {
    fn new() -> Self;
    fn solve(&self, puzzle_type: PuzzleType);
}

struct DayOne {
    day: u32,
}

impl DayOne {

    fn get_fuel_required_using_mass(mass: i32) -> i32 {
        let div = mass as f32 / 3.0;
        let fuel_required = div.floor() as i32 - 2;
        
        fuel_required
    }

    fn get_fuel_required_using_mass_bonus(mass: i32) -> i32 {
        // fuel now also requires fuel so we keep rounding down
        // until we're at 0
        let mut total_fuel_required = 0;
        let mut current_mas = mass;

        loop {
            let div = current_mas as f32 / 3.0;
            let fuel_required = div.floor() as i32 - 2;
            
            if fuel_required <= 0 {
                break 
            }
            total_fuel_required += fuel_required;
            current_mas = fuel_required;
        }

        total_fuel_required
    }
}

impl Puzzle for DayOne {
    fn new() -> Self {
        DayOne{ day: 1 }
    }

    fn solve(&self, puzzle_type: PuzzleType) {
        let input = PuzzleInput::new(self.day);

        match puzzle_type {
            PuzzleType::PartOne => {
                let fuel_sum: i32 = input.puzzle_input.split('\n')
                    .map(|x| x.parse::<i32>().unwrap())
                    .map(|n| DayOne::get_fuel_required_using_mass(n))
                    .sum();

                println!("Fuel sum: {fuel_sum}");
            }
            PuzzleType::PartTwo => {
                let fuel_sum: i32 = input.puzzle_input.split('\n')
                    .map(|x| x.parse::<i32>().unwrap())
                    .map(|n| DayOne::get_fuel_required_using_mass_bonus(n))
                    .sum();

                println!("Fuel sum: {fuel_sum}");
            }
        }
    }
}


fn main() {

    let puzzle = DayOne::new();
    puzzle.solve(PuzzleType::PartOne);
    puzzle.solve(PuzzleType::PartTwo);
}
