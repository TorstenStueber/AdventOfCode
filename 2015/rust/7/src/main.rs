use std::collections::HashMap;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
enum Instruction {
    InputDirect(u16),
    InputWire(String),
    And(String, String),
    Or(String, String),
    LShift(String, u8),
    RShift(String, u8),
    Not(String),
}

use Instruction::*;

type Input = Vec<(String, Instruction)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (lhs, rhs) = line.split_once("->").unwrap();
            let rhs = rhs.trim().into();
            if let Some((op1, op2)) = lhs.split_once("AND") {
                (rhs, And(op1.trim().into(), op2.trim().into()))
            } else if let Some((op1, op2)) = lhs.split_once("OR") {
                (rhs, Or(op1.trim().into(), op2.trim().into()))
            } else if let Some((op1, op2)) = lhs.split_once("LSHIFT") {
                (rhs, LShift(op1.trim().into(), op2.trim().parse().unwrap()))
            } else if let Some((op1, op2)) = lhs.split_once("RSHIFT") {
                (rhs, RShift(op1.trim().into(), op2.trim().parse().unwrap()))
            } else if let Some((_, op)) = lhs.split_once("NOT") {
                (rhs, Not(op.trim().into()))
            } else {
                if let Ok(signal) = lhs.trim().parse() {
                    (rhs, InputDirect(signal))
                } else {
                    (rhs, InputWire(lhs.trim().into()))
                }
            }
        })
        .collect()
}

fn execute<'a>(mut result: HashMap<&'a String, u16>, input: &'a Input, target: String) -> u16 {
    let get = |result: &HashMap<_, _>, key: &String| -> Option<u16> {
        if let Ok(signal) = key.parse() {
            Some(signal)
        } else {
            result.get(key).map(|a| *a)
        }
    };

    while !result.contains_key(&target) {
        for (wire, instruction) in input {
            if !result.contains_key(wire) {
                match instruction {
                    Instruction::InputDirect(signal) => {
                        result.insert(wire, *signal);
                    }
                    Instruction::InputWire(op) => {
                        if let Some(signal) = get(&result, op) {
                            result.insert(wire, signal);
                        }
                    }
                    And(op1, op2) => {
                        if let (Some(signal1), Some(signal2)) =
                            (get(&result, op1), get(&result, op2))
                        {
                            result.insert(wire, signal1 & signal2);
                        }
                    }
                    Or(op1, op2) => {
                        if let (Some(signal1), Some(signal2)) =
                            (get(&result, op1), get(&result, op2))
                        {
                            result.insert(wire, signal1 | signal2);
                        }
                    }
                    LShift(op1, op2) => {
                        if let Some(signal1) = get(&result, op1) {
                            result.insert(wire, signal1 << op2);
                        }
                    }
                    RShift(op1, op2) => {
                        if let Some(signal1) = get(&result, op1) {
                            result.insert(wire, signal1 >> op2);
                        }
                    }
                    Not(op) => {
                        if let Some(signal) = get(&result, op) {
                            result.insert(wire, !signal);
                        }
                    }
                };
            }
        }
    }

    *result.get(&target).unwrap()
}

fn task1(input: &Input, target: String) -> u16 {
    let result = HashMap::new();

    execute(result, input, target)
}

fn task2(input: &Input) -> u16 {
    let result = HashMap::new();

    let signal_a = execute(result, input, "a".into());

    let mut result = HashMap::new();
    let b = String::from("b");
    result.insert(&b, signal_a);

    execute(result, input, "a".into())
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input, "a".into()));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";

    let input = parse_input(&test_input);

    assert_eq!(task1(&input, "h".into()), 65412);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input, "a".into()), 956);
    assert_eq!(task2(&input), 40149);
}
