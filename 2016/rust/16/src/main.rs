use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<bool>;

fn parse_input(input: &str) -> Input {
    input.trim().chars().map(|c| c == '1').collect()
}

fn execute(input: &Input, max_length: usize) -> String {
    let mut input = input.clone();

    while input.len() < max_length {
        input = input
            .clone()
            .into_iter()
            .chain(std::iter::once(false))
            .chain(input.into_iter().rev().map(|b| !b))
            .collect()
    }

    input.truncate(max_length);
    while input.len() % 2 == 0 {
        input = input.chunks(2).map(|c| c[0] == c[1]).collect();
    }

    input
        .into_iter()
        .map(|b| if b { '1' } else { '0' })
        .collect()
}

fn task1(input: &Input) -> String {
    execute(&input, 272)
}

fn task2(input: &Input) -> String {
    execute(&input, 35651584)
}

fn main() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        10000
        ";
    let input = parse_input(input);

    assert_eq!(execute(&input, 20), "01100");
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), "10010110010011110");
    assert_eq!(task2(&input), "01101011101100011");
}
