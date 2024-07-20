use crate::utils::Puzzle;

pub fn solve_day_one(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

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
