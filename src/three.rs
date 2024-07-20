use std::{num::ParseIntError, ops, str::FromStr};
use crate::utils::Puzzle;

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

impl Point {
    fn euclid(&self, p: &Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
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
    end: Point
}

impl Line {
    fn from_instruction(ins: &Instruction, pos: Point) -> Self {
        let dir = match ins.direction {
            Dir::UP => Vec2::from_tuple(UP),
            Dir::DOWN => Vec2::from_tuple(DOWN),
            Dir::LEFT => Vec2::from_tuple(LEFT),
            Dir::RIGHT => Vec2::from_tuple(RIGHT)
        };
        Self {
            start: pos,
            end: pos + Point::from(dir.scale(ins.steps))
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
                Some(line ) => Point::from(line.end)
            };

            let line = Line::from_instruction(ins, pos);
            acc.push(line);

            acc
        });

        Wire{ lines }
    }
}

#[derive(PartialEq)]
enum Orient {
    VERT,
    HOR,
}

fn check_intersect(a: &Line, b: &Line) -> Option<(Orient, Orient)> {
    let slope_a = (a.end.y - a.start.y).checked_div(a.end.x - a.start.x);
    let slope_b = (b.end.y - b.start.y).checked_div(b.end.x - b.start.x);

    match (slope_a, slope_b) {
        // paralel lines
        (None, None) => None,
        (Some(x), Some(y)) if x == y => None,

        // a is vertical
        (None, Some(_)) if (
            a.start.x >= b.start.x.min(b.end.x) &&
            a.start.x <= b.start.x.max(b.end.x) &&
            a.start.y.min(a.end.y) <= b.start.y &&
            a.start.y.max(a.end.y) >= b.start.y
        ) => Some((Orient::VERT, Orient::HOR)),

        // b is vertical
        (Some(_), None) if (
            b.start.x >= a.start.x.min(a.end.x) &&
            b.start.x <= a.start.x.max(a.end.x) &&
            b.start.y.min(b.end.y) <= a.start.y &&
            b.start.y.max(b.end.y) >= a.start.y

        ) => Some((Orient::HOR, Orient::VERT)),

        // might cross but not within bounds
        _ => None
    }
}

pub fn solve_day_three(puzzle: &Puzzle) -> (Option<String>, Option<String>) {

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

        // for each line in the first wire, check in the second if they intersect
        let mut intersections: Vec<Point> = vec![];
        for a in &wires[0].lines {
            for b in &wires[1].lines {
                let opt_intersect = match check_intersect(a, b) {
                    Some((Orient::HOR, Orient::VERT)) => Some(Point{x: b.start.x, y: a.start.y}),
                    Some((Orient::VERT, Orient::HOR)) => Some(Point{x: a.start.x, y: b.start.y}),
                    _ => None
                };
                if let Some(p) = opt_intersect {
                    intersections.push(p);
                }
            }
        }

        let min = intersections.iter().filter(|p| !(p.x == 0 && p.y == 0)).map(|p| p.x.abs() + p.y.abs()).min().unwrap();

        println!("{}", min);

        // dbg!(&wires[0].lines);
        // dbg!(intersections);
        
        None
    }

    (part_one(&puzzle), None)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_intersect_a_vert() {
        let a = Line{
            start: Point{ x: 4, y: 2},
            end: Point{ x: 2, y: 2},
        };
        let b = Line{
            start: Point{ x: 3, y: 1},
            end: Point{ x: 3, y: 3},
        };
        assert!(check_intersect(&a, &b) == Some((Orient::HOR, Orient::VERT)));
    }

    #[test]
    fn test_check_intersect_b_vert() {
        let a = Line{
            start: Point{ x: 4, y: 2},
            end: Point{ x: 2, y: 2},
        };
        let b = Line{
            start: Point{ x: 3, y: 1},
            end: Point{ x: 3, y: 3},
        };
        assert!(check_intersect(&b, &a) == Some((Orient::VERT, Orient::HOR)));
    }

    #[test]
    fn test_check_intersect_a_vert_switch() {
        let a = Line{
            start: Point{ x: 2, y: 2},
            end: Point{ x: 4, y: 2},
        };
        let b = Line{
            start: Point{ x: 3, y: 3},
            end: Point{ x: 3, y: 1},
        };
        assert!(check_intersect(&a, &b) == Some((Orient::HOR, Orient::VERT)));
    }

    #[test]
    fn test_check_intersect_b_vert_switch() {
        let a = Line{
            start: Point{ x: 4, y: 2},
            end: Point{ x: 2, y: 2},
        };
        let b = Line{
            start: Point{ x: 3, y: 1},
            end: Point{ x: 3, y: 3},
        };
        assert!(check_intersect(&b, &a) == Some((Orient::VERT, Orient::HOR)));
    }
}
