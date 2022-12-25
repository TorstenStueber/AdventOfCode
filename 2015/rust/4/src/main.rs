use md5::{Digest, Md5};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    String::from(input.trim())
}

fn task1(input: &Input) -> usize {
    let mut counter = 1;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input);
        hasher.update(counter.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] <= 0x0f {
            return counter;
        }

        counter += 1;
    }
}

fn task2(input: &Input) -> usize {
    let mut counter = 1;
    let mut hasher = Md5::new();
    loop {
        hasher.update(input);
        hasher.update(counter.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return counter;
        }

        counter += 1;
    }
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = parse_input("abcdef");
    let test_input2 = parse_input("pqrstuv");

    assert_eq!(task1(&test_input1), 609043);
    assert_eq!(task1(&test_input2), 1048970);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 282749);
    assert_eq!(task2(&input), 9962624);
}
