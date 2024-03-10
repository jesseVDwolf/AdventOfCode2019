use std::fs;
use std::path::{Path, PathBuf};
use std::env;

fn read_puzzle_input(path: PathBuf) -> String {
    let puzzle_input = fs::read_to_string(path).unwrap();

    puzzle_input
}

fn get_fuel_required_using_mass(mass: i32) -> i32 {
    let div = mass as f32 / 3.0;
    let fuel_required = div.floor() as i32 - 2;
    
    fuel_required
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(Path::new("static/inputs/day1.txt"));
    let puzzle_input = read_puzzle_input(file_path);

    let fuel_sum: i32 = puzzle_input.split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .map(|n| get_fuel_required_using_mass(n))
        .sum();

    println!("File path: {}.", puzzle_input);
    println!("Fuel sum: {}.", fuel_sum);
}
