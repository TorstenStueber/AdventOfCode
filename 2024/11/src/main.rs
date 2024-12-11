use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<usize>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn next_numbers(number: usize) -> Vec<usize> {
    match number {
        0 => vec![1],
        _ if format!("{}", number).len() % 2 == 0 => {
            let str = format!("{}", number);
            vec![
                str[..str.len() / 2].parse().unwrap(),
                str[str.len() / 2..].parse().unwrap(),
            ]
        }
        _ => vec![number * 2024],
    }
}

fn simple_expand(input: &Input, steps: usize) -> usize {
    let mut current = input.clone();
    for _ in 1..=steps {
        current = current.iter().map(|a| next_numbers(*a)).flatten().collect()
    }

    current.len()
}

fn task1(input: &Input) -> usize {
    simple_expand(input, 25)
}

fn task2(input: &Input) -> usize {
    let mut todos = input.clone();
    let mut done = HashSet::new();

    for number in input.iter() {
        done.insert(*number);
    }

    while !todos.is_empty() {
        let mut new_todos = Vec::new();
        for todo in todos {
            let next_steps = next_numbers(todo);
            for next_step in next_steps {
                if !done.contains(&next_step) {
                    new_todos.push(next_step);
                    done.insert(next_step);
                }
            }
        }
        todos = new_todos;
    }

    let mut mapping = HashMap::new();
    for number in done.iter() {
        mapping.insert(number, 1);
    }

    for _ in 1..=75 {
        let mut new_mapping = HashMap::new();
        for number in done.iter() {
            let next_steps = next_numbers(*number);

            new_mapping.insert(
                number,
                next_steps
                    .iter()
                    .fold(0, |acc, step| acc + mapping.get(step).unwrap()),
            );
        }
        mapping = new_mapping;
    }

    input
        .iter()
        .fold(0, |acc, step| acc + mapping.get(step).unwrap())
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        0 1 10 99 999";

    let test_input2 = "
        125 17";

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);

    assert_eq!(simple_expand(&input1, 1), 7);
    assert_eq!(simple_expand(&input2, 6), 22);
    assert_eq!(simple_expand(&input2, 25), 55312);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 229043);
    assert_eq!(task2(&input), 272673043446478);
}
