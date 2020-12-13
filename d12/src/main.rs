use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use aoc_2020::get_input;
use num_complex::Complex;

struct ParseInstructionError {}

trait ToComplex<T> {
    fn to_complex(&self) -> Complex<i32> {
        Complex::new(0, 0)
    }

    fn from_complex(c: Complex<i32>) -> T;
}

#[derive(Debug)]
enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl ToComplex<Instruction> for Instruction {
    fn to_complex(&self) -> Complex<i32> {
        match self {
            Instruction::North(_) => Complex::new(0, 1),

            Instruction::South(_) => Complex::new(0, -1),
            Instruction::East(_) => Complex::new(1, 0),
            Instruction::West(_) => Complex::new(-1, 0),
            Instruction::Left(_) => Complex::new(0, 1),
            Instruction::Right(_) => Complex::new(0, -1),
            Instruction::Forward(_) => Complex::new(0, 0),
        }
    }
    fn from_complex(c: Complex<i32>) -> Instruction {
        match c {
            Complex { re: 1, im: 0 } => Instruction::East(0),
            Complex { re: 0, im: 1 } => Instruction::North(0),
            Complex { re: -1, im: 0 } => Instruction::West(0),
            Complex { re: 0, im: -1 } => Instruction::South(0),
            _ => Instruction::East(0),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let instr = match chars.next() {
            Some(c) => match c {
                'N' => Instruction::North,
                'S' => Instruction::South,
                'E' => Instruction::East,
                'W' => Instruction::West,
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                'F' => Instruction::Forward,
                _ => return Err(ParseInstructionError {}),
            },
            None => return Err(ParseInstructionError {}),
        };

        match chars.as_str().parse::<u32>() {
            Ok(c) => Ok(instr(c)),
            Err(_) => Err(ParseInstructionError {}),
        }
    }
}

fn parse(input: &[String]) -> Vec<Instruction> {
    input
        .iter()
        .map(|i| match Instruction::from_str(i.as_str()) {
            Ok(i) => i,
            Err(_) => panic!("Unable to parse {}", i),
        })
        .collect()
}

fn part1(instr: &Vec<Instruction>) -> (u32, u32) {
    let mut position1: Complex<i32> = Complex::new(0, 0);
    let mut position2: Complex<i32> = Complex::new(0, 0);
    let mut current_dir = Instruction::East(0);
    let mut waypoint: Complex<i32> = Complex::new(10, 1);

    for i in instr {
        match i {
            Instruction::North(c)
            | Instruction::East(c)
            | Instruction::South(c)
            | Instruction::West(c) => {
                let action = i.to_complex().scale(*c as i32);
                position1 += action;
                waypoint += action
            }
            Instruction::Forward(c) => {
                let action = current_dir.to_complex().scale(*c as i32);
                position1 += action;
                position2 += waypoint.scale(*c as i32)
            }
            Instruction::Left(c) | Instruction::Right(c) => {
                let turn = i.to_complex().powi((c / 90) as i32);
                current_dir = Instruction::from_complex(current_dir.to_complex() * turn);
                waypoint *= turn
            }
        }
    }
    (position1.l1_norm() as u32, position2.l1_norm() as u32)
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let instr = parse(input);
    let (p1, p2) = part1(&instr);
    // let p2 = part2(&instr);

    (p1, p2)
}

fn main() {
    let input = get_input("d12.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}
