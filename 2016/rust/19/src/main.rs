use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = u64;

fn parse_input(input: &str) -> Input {
    input.trim().parse().unwrap()
}

fn rec(elements: u64) -> u64 {
    if elements == 1 {
        0
    } else if elements % 2 == 0 {
        rec(elements >> 1) << 1
    } else {
        (rec(elements >> 1) << 1) + 2
    }
}

fn task1(input: &Input) -> u64 {
    rec(*input) + 1
}

fn rec2(elements: u64) -> u64 {
    if elements == 1 {
        0
    } else if elements % 2 == 0 {
        let result = rec(elements >> 1 + elements / 6);
        if result < elements >> 1 {
            result
        } else {
            (result - elements >> 1 + 1) * 3 + elements >> 1 - 1
        }
    } else {
        let result = rec(elements >> 1 + elements / 6);
        let result = (rec(elements >> 1) << 1) + 2
    }
}

fn task2(input: &Input) -> usize {
    rec2(*input) + 1
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input("5");
    assert_eq!(task1(&input), 3);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    // assert_eq!(task1(&input), 2005);
    // assert_eq!(task2(&input), 20008491);
}
