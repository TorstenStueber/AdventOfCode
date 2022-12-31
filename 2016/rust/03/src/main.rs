use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(u32, u32, u32)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut iter = line
                .trim()
                .split_whitespace()
                .map(|number| number.parse().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect()
}

fn task1(input: &Input) -> u32 {
    let mut correct = 0;
    for (a, b, c) in input {
        if a + b > *c && a + c > *b && b + c > *a {
            correct += 1;
        }
    }

    correct
}

fn task2(input: &Input) -> u32 {
    let mut lengths = vec![];
    for (l, _, _) in input {
        lengths.push(*l);
    }
    for (_, l, _) in input {
        lengths.push(*l);
    }
    for (_, _, l) in input {
        lengths.push(*l);
    }

    let mut iter = lengths.into_iter().peekable();
    let mut new_input = vec![];

    while iter.peek().is_some() {
        new_input.push((
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ));
    }

    task1(&new_input)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 862);
    assert_eq!(task2(&input), 1577);
}
