use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

use Instruction::*;

type Input = Vec<(Instruction, (u32, u32), (u32, u32))>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut trimmed = line.trim();
            let instruction = if let Some((_, s)) = trimmed.split_once("toggle") {
                trimmed = s.into();
                Toggle
            } else if let Some((_, s)) = trimmed.split_once("turn on") {
                trimmed = s.into();
                TurnOn
            } else if let Some((_, s)) = trimmed.split_once("turn off") {
                trimmed = s.into();
                TurnOff
            } else {
                panic!()
            };

            let coordinates = trimmed.trim().split_once(" through ").unwrap();
            let begin = coordinates.0.split_once(",").unwrap();
            let end = coordinates.1.split_once(",").unwrap();

            (
                instruction,
                (begin.0.parse().unwrap(), begin.1.parse().unwrap()),
                (end.0.parse().unwrap(), end.1.parse().unwrap()),
            )
        })
        .collect()
}

fn task1(input: &Input) -> usize {
    let mut field = HashSet::new();
    for (instruction, (x1, y1), (x2, y2)) in input {
        let (x1, x2) = (*x1.min(x2), *x1.max(x2));
        let (y1, y2) = (*y1.min(y2), *y1.max(y2));

        for x in x1..=x2 {
            for y in y1..=y2 {
                match instruction {
                    Toggle => {
                        if field.contains(&(x, y)) {
                            field.remove(&(x, y))
                        } else {
                            field.insert((x, y))
                        }
                    }
                    TurnOff => field.remove(&(x, y)),
                    TurnOn => field.insert((x, y)),
                };
            }
        }
    }

    field.len()
}

fn task2(input: &Input) -> usize {
    let mut field = HashMap::new();
    for (instruction, (x1, y1), (x2, y2)) in input {
        let (x1, x2) = (*x1.min(x2), *x1.max(x2));
        let (y1, y2) = (*y1.min(y2), *y1.max(y2));

        for x in x1..=x2 {
            for y in y1..=y2 {
                match instruction {
                    Toggle => {
                        field.entry((x, y)).and_modify(|e| *e += 2).or_insert(2);
                    }
                    TurnOff => {
                        field.entry((x, y)).and_modify(|e| {
                            if *e > 0 {
                                *e -= 1
                            }
                        });
                    }
                    TurnOn => {
                        field.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
                    }
                };
            }
        }
    }

    field.iter().fold(0, |acc, (_, v)| acc + v)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
    turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500";

    let input = parse_input(&test_input);

    assert_eq!(task1(&input), 1000 * 999 - 4);

    let test_input = "
    turn on 0,0 through 0,0
    toggle 0,0 through 999,999";

    let input = parse_input(&test_input);

    assert_eq!(task2(&input), 2000001);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 543903);
    assert_eq!(task2(&input), 14687245);
}
