use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<u8>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|char| String::from(char).parse().unwrap())
        .collect()
}

fn iterate(input: &Input, iterations: u8) -> usize {
    let mut current = input.clone();
    for _ in 0..iterations {
        let mut next = vec![];
        let mut i = 0;
        while i < current.len() {
            let n = current[i];
            let mut counter = 0;
            while i < current.len() && current[i] == n {
                counter += 1;
                i += 1;
            }
            next.push(counter);
            next.push(n);
        }

        current = next;
    }

    current.len()
}

fn task1(input: &Input) -> usize {
    iterate(&input, 40)
}

fn task2(input: &Input) -> usize {
    iterate(&input, 50)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "1";

    let input = parse_input(&test_input);

    assert_eq!(iterate(&input, 5), 6);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 252594);
    assert_eq!(task2(&input), 3579328);
}
