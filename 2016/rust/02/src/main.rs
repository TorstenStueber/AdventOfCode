use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

enum Instruction {
    U,
    D,
    L,
    R,
}

use Instruction::*;

type Input = Vec<Vec<Instruction>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| match char {
                    'U' => U,
                    'D' => D,
                    'L' => L,
                    'R' => R,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn task1(input: &Input) -> i32 {
    let (mut x, mut y) = (1i32, 1i32);
    let mut result = 0;

    for line in input {
        for instruction in line {
            let (x_new, y_new) = match instruction {
                U => (x, y - 1),
                D => (x, y + 1),
                L => (x - 1, y),
                R => (x + 1, y),
            };

            if x_new >= 0 && x_new <= 2 && y_new >= 0 && y_new <= 2 {
                (x, y) = (x_new, y_new);
            }
        }
        let digit = 1 + y * 3 + x;
        result = result * 10 + digit;
    }

    result
}

fn task2(input: &Input) -> String {
    let (mut x, mut y) = (0i32, 2i32);
    let mut result = String::new();

    for line in input {
        for instruction in line {
            let (x_new, y_new) = match instruction {
                U => (x, y - 1),
                D => (x, y + 1),
                L => (x - 1, y),
                R => (x + 1, y),
            };

            if (x_new - 2).abs() + (y_new - 2).abs() <= 2 {
                (x, y) = (x_new, y_new);
            }
        }

        let char = match (x, y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => unreachable!(),
        };
        result.push(char);
    }

    result
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        ULL
        RRDDD
        LURDL
        UUUUD";
    let input = parse_input(input);

    assert_eq!(task1(&input), 1985);
    assert_eq!(task2(&input), String::from("5DB3"));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 95549);
    assert_eq!(task2(&input), String::from("D87AD"));
}
