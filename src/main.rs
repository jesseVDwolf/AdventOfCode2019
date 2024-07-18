use std::{env, fs, io};


struct Puzzle {
    text: String,
}

impl Puzzle {
    pub fn from_puzzle_file(day: i16) -> Result<Self, io::Error> {
        let path = env::current_dir()?
            .join("static")
            .join("inputs")
            .join("day".to_string() + &day.to_string() + &".txt");

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

    fn part_one(p: &Puzzle) -> Option<String> {
        let mut numbers = p
            .text
            .split_terminator(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        numbers[1] = 12;
        numbers[2] = 2;
        for i in 0..numbers.len() {
            if i % 4 > 0 {
                continue;
            }
            let index_a = i + 1;
            let index_b = i + 2;
            let index_c = i + 3;
            if numbers[index_c] == 0 {
                println!("[{}] {}", i, numbers[i]);
            }
            match numbers[i] {
                99 => break,
                1 => numbers[index_c] = numbers[index_a] + numbers[index_b],
                2 => numbers[index_c] = numbers[index_a] * numbers[index_b],
                _ => panic!("Unknown number")
            }
        }

        Some(numbers[0].to_string())
    }

    (part_one(&puzzle), None)
}

fn main() -> Result<(), io::Error>{
    let solvers: Vec<Solver> = vec![
        solve_day_one,
        solve_day_two
    ];
    let mut last_day = 2;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        last_day = args[1].parse::<i16>().unwrap();
    }
    let day_range = 1..=last_day;

    for (day, solver) in day_range.zip(solvers) {
        let puzzle = Puzzle::from_puzzle_file(day)?;
        
        match solver(&puzzle) {
            (None, None) => println!("No puzzle solved for day {}!", day),
            (Some(v), None) => println!("[{}] Part one: {}", day, v),
            (None, Some(v)) => println!("[{}] Part two: {}", day, v),
            (Some(a), Some(b)) => println!("[{}] Part one: {}\n[{}] Part two: {}", day, a, day, b)
        }
    }

    Ok(())
}
