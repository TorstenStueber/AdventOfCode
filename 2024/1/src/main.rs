use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = (Vec<i32>, Vec<i32>);

fn parse_input(input: &str) -> Input {
    let parsed = input.trim().lines().map(|line| {
        line.trim()
            .split_ascii_whitespace()
            .map(|number| number.parse().unwrap())
            .collect::<Vec<i32>>()
    });

    (
        parsed.clone().map(|line| line[0]).collect(),
        parsed.map(|line| line[1]).collect(),
    )
}

fn task1(input: &Input) -> i32 {
    let (mut left, mut right) = (input.0.clone(), input.1.clone());
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn task2(input: &Input) -> i32 {
    let (mut left, mut right) = (input.0.clone(), input.1.clone());
    left.sort();
    right.sort();

    let mut result = 0;
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        let current = left[left_index];
        let mut left_count = 0;
        let mut right_count = 0;

        while left_index < left.len() && left[left_index] == current {
            left_count += 1;
            left_index += 1;
        }

        while right_index < right.len() && right[right_index] <= current {
            if right[right_index] == current {
                right_count += 1;
            }
            right_index += 1;
        }

        result += left_count * right_count * current;
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
    let test_input = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 11);
    assert_eq!(task2(&input), 31);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 2264607);
    assert_eq!(task2(&input), 19457120);
}
