use std::collections::HashSet;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

enum Instruction {
    L(i32),
    R(i32),
}

use Instruction::*;

type Input = Vec<Instruction>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split(",")
        .map(|mut line| {
            line = line.trim();
            if let Some(length) = line.split_once("L") {
                L(length.1.trim().parse().unwrap())
            } else if let Some(length) = line.split_once("R") {
                R(length.1.trim().parse().unwrap())
            } else {
                panic!()
            }
        })
        .collect()
}

fn task1(input: &Input) -> i32 {
    let mut dir = 0;
    let (mut x, mut y) = (0, 0);
    for instruction in input {
        let length = match instruction {
            L(length) => {
                dir -= 1;
                length
            }
            R(length) => {
                dir += 1;
                length
            }
        };

        match (dir % 4 + 4) % 4 {
            0 => y -= length,
            1 => x += length,
            2 => y += length,
            3 => x -= length,
            _ => unreachable!(),
        }
    }

    x.abs() + y.abs()
}

fn task2(input: &Input) -> i32 {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut dir = 0;
    let (mut x, mut y) = (0i32, 0i32);
    for instruction in input {
        let length = match instruction {
            L(length) => {
                dir -= 1;
                length
            }
            R(length) => {
                dir += 1;
                length
            }
        };

        for _ in 0..*length {
            match (dir % 4 + 4) % 4 {
                0 => y -= 1,
                1 => x += 1,
                2 => y += 1,
                3 => x -= 1,
                _ => unreachable!(),
            }

            if visited.contains(&(x, y)) {
                return x.abs() + y.abs();
            }

            visited.insert((x, y));
        }
    }

    unreachable!()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input1 = parse_input("R2, L3");
    let input2 = parse_input("R2, R2, R2");
    let input3 = parse_input("R5, L5, R5, R3");

    assert_eq!(task1(&input1), 5);
    assert_eq!(task1(&input2), 2);
    assert_eq!(task1(&input3), 12);

    let input1 = parse_input("R8, R4, R4, R8");
    assert_eq!(task2(&input1), 4);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 278);
    assert_eq!(task2(&input), 161);
}
