use std::collections::HashSet;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<char>;

fn parse_input(input: &str) -> Input {
    input.trim().chars().collect()
}

fn task1(input: &Input) -> usize {
    let mut position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(position);

    for c in input {
        match c {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 -= 1,
            '^' => position.1 += 1,
            _ => panic!(),
        }
        visited.insert(position);
    }

    visited.len()
}

fn task2(input: &Input) -> usize {
    let mut positions = vec![(0, 0), (0, 0)];
    let mut index = 0;
    let mut visited = HashSet::new();

    visited.insert(positions[0]);

    for c in input {
        match c {
            '<' => positions[index].0 -= 1,
            '>' => positions[index].0 += 1,
            'v' => positions[index].1 -= 1,
            '^' => positions[index].1 += 1,
            _ => panic!(),
        }
        visited.insert(positions[index]);

        index = 1 - index;
    }

    visited.len()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = parse_input(">");
    let test_input2 = parse_input("^>v<");
    let test_input3 = parse_input("^v^v^v^v^v");

    assert_eq!(task1(&test_input1), 2);
    assert_eq!(task1(&test_input2), 4);
    assert_eq!(task1(&test_input3), 2);

    let test_input1 = parse_input("^v");
    let test_input2 = parse_input("^>v<");
    let test_input3 = parse_input("^v^v^v^v^v");

    assert_eq!(task2(&test_input1), 3);
    assert_eq!(task2(&test_input2), 3);
    assert_eq!(task2(&test_input3), 11);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 2592);
    assert_eq!(task2(&input), 2360);
}
