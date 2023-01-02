use md5::{Digest, Md5};
use std::{char::from_digit, collections::HashMap, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    input.trim().into()
}

fn task1(input: &Input) -> String {
    let mut counter = 0;
    let mut hasher = Md5::new();
    let mut string = String::new();
    loop {
        hasher.update(input);
        hasher.update(counter.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] <= 0x0f {
            string.push(from_digit(result[2] as u32, 16).unwrap());
            if string.len() == 8 {
                return string;
            }
        }

        counter += 1;
    }
}

fn task2(input: &Input) -> String {
    let mut counter = 0;
    let mut hasher = Md5::new();
    let mut chars = HashMap::new();
    loop {
        hasher.update(input);
        hasher.update(counter.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] < 0x08 {
            if !chars.contains_key(&result[2]) {
                chars.insert(result[2], from_digit((result[3] >> 4) as u32, 16).unwrap());
                if chars.len() == 8 {
                    break;
                }
            }
        }

        counter += 1;
    }
    let mut string = String::new();
    for i in 0..8 {
        string.push(*chars.get(&i).unwrap());
    }

    string
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        abc";
    let input = parse_input(input);

    assert_eq!(task1(&input), "18f47a30");
    assert_eq!(task2(&input), "05ace8e3");
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), String::from("f77a0e6e"));
    assert_eq!(task2(&input), String::from("999828ec"));
}
