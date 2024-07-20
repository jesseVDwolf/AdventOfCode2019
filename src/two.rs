use itertools::iproduct;

use crate::utils::Puzzle;


pub fn solve_day_two(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

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