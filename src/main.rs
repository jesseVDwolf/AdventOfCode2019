use std::{env, fs, io, num::ParseIntError, str::FromStr, string::ParseError};
use std::ops;

use clap::Parser;
use itertools::iproduct;


struct Puzzle {
    text: String,
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


type SolveResult = (Option<String>, Option<String>);
type Solver = fn(&Puzzle) -> SolveResult;

fn solve_day_one(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

    fn part_one(p: &Puzzle) -> Option<String> {
        let s = p
            .text
            .split_terminator('\n')
            .fold(0, |acc, x| acc + ((x.parse::<f64>().unwrap() / 3.0).floor() as u64 - 2));
        
        Some(s.to_string())
    }

    fn part_two(p: &Puzzle) -> Option<String> {

        fn reduce(a: i64, n: i64) -> i64 {
            let x = (n as f64 / 3.0).floor() as i64 - 2;
            if x < 0 { a } else { reduce(a + x, x) }
        }

        let s = p
            .text
            .split_terminator('\n')
            .fold(0, |acc, x| acc + reduce(0, x.parse().unwrap()));

        Some(s.to_string())
    }

    (part_one(&puzzle), part_two(&puzzle))
}

fn solve_day_two(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

    fn get_program_output(numbers: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
        numbers[1] = noun;
        numbers[2] = verb;
        'main: for i in 0..numbers.len() {
            if i % 4 > 0 {
                continue;
            }
            let index_a = numbers[i + 1];
            let index_b = numbers[i + 2];
            let index_c = numbers[i + 3];
            match numbers[i] {
                99 => break 'main,
                1 => numbers[index_c] = numbers[index_a] + numbers[index_b],
                2 => numbers[index_c] = numbers[index_a] * numbers[index_b],
                _ => panic!("Unknown number")
            }
        }

        numbers[0]
    }

    fn part_one(p: &Puzzle) -> Option<String> {
        let mut numbers = p
            .text
            .split_terminator(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Some(get_program_output(&mut numbers, 12, 2).to_string())
    }

    fn part_two(p: &Puzzle) -> Option<String> {
        let mut numbers = p
            .text
            .split_terminator(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        let range = 0..=99;
        for (noun, verb) in iproduct!(range.clone(), range.clone()) {
            let mut n = numbers.clone();
            let output = get_program_output(&mut n, noun, verb);
            if output == 19690720 {
                return Some((100 * noun + verb).to_string())
            }
        }

        None
    }

    (part_one(&puzzle), part_two(&puzzle))
}


const UP: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<Vec2> for Point {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn scale(&self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    fn from_tuple(tup: (i32, i32)) -> Self {
        Self {
            x: tup.0,
            y: tup.1
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    dir: Vec2
}

impl Line {
    fn from_instruction(ins: &Instruction, pos: Point) -> Self {
        let dir = match ins.direction {
            Dir::UP => Vec2::from_tuple(UP),
            Dir::DOWN => Vec2::from_tuple(UP),
            Dir::LEFT => Vec2::from_tuple(LEFT),
            Dir::RIGHT => Vec2::from_tuple(RIGHT)
        };
        Self {
            start: pos,
            dir: dir.scale(ins.steps)
        }
    }

    fn port() -> Self {
        Self {
            start: Point{ x: 0, y: 0},
            dir: Vec2{ x: 0, y: 0}
        }
    }
}

#[derive(Debug)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct Instruction {
    direction: Dir,
    steps: i32
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // example U28,R14,L127,D87
        let (a, b) = s.split_at(1);
        let steps = b.parse::<i32>()?;
        match a {
            "U" => Ok(Instruction{direction: Dir::UP, steps}),
            "D" => Ok(Instruction{direction: Dir::DOWN, steps}),
            "L" => Ok(Instruction{direction: Dir::LEFT, steps}),
            "R" => Ok(Instruction{direction: Dir::RIGHT, steps}),
            _ => panic!("Unknown direction!")
        }
    }
}

#[derive(Debug)]
struct Wire {
    lines: Vec<Line>
}

impl Wire {
    fn from_instruction_list(instructions: &Vec<Instruction>) -> Self {
        let lines = instructions.iter().fold(Vec::new(), |mut acc: Vec<Line>, ins| {
            let pos: Point = match acc.last() {
                None => Point{x: 0, y: 0},
                Some(line ) => line.start + Point::from(line.dir)
            };

            let line = Line::from_instruction(ins, pos);
            acc.push(line);

            acc
        });

        Wire{ lines }
    }
}

fn solve_day_three(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

    fn part_one(p: &Puzzle) -> Option<String> {
        let instructions: Vec<Vec<Instruction>> = p
            .text
            .split_terminator('\n')
            .map(|x| {
                x
                .split_terminator(',')
                .map(|a| a.parse::<Instruction>().unwrap())
                .collect::<Vec<Instruction>>()
            })
            .collect();
        
        // create the 2 wires
        let wires = instructions.iter().map(Wire::from_instruction_list).collect::<Vec<_>>();

        dbg!(&wires[0].lines[..3]);
        
        None
    }

    (part_one(&puzzle), None)
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    
    #[arg(short, long, default_value_t = 25)]
    day: i16,

    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn main() -> Result<(), io::Error>{
    let solvers: Vec<Solver> = vec![
        solve_day_one,
        solve_day_two,
        solve_day_three
    ];
    // let mut last_day = 3;
    // let args: Vec<_> = env::args().collect();
    // if args.len() > 1 {
    //     last_day = args[1].parse::<i16>().unwrap();
    // }
    let args = Args::parse();
    let day_range = 1..=args.day;

    for (day, solver) in day_range.zip(solvers) {
        let puzzle = Puzzle::from_puzzle_file(day, args.example);
        if puzzle.is_err() {
            continue;
        }
        
        match solver(&puzzle.unwrap()) {
            (None, None) => println!("No puzzle solved for day {}!", day),
            (Some(v), None) => println!("[{}] Part one: {}", day, v),
            (None, Some(v)) => println!("[{}] Part two: {}", day, v),
            (Some(a), Some(b)) => println!("[{}] Part one: {}\n[{}] Part two: {}", day, a, day, b)
        }
    }

    Ok(())
}
