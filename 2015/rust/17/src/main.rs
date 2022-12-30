use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<u32>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn task1(input: &Input, target: u32) -> u32 {
    let mut choices: Vec<_> = input.iter().map(|_| false).collect();

    let mut count = 0;
    loop {
        let sum = input.iter().enumerate().fold(
            0,
            |acc, (index, size)| if choices[index] { acc + size } else { acc },
        );

        if sum == target {
            count += 1;
        }

        let mut done = true;
        for choice in choices.iter_mut() {
            if *choice == false {
                *choice = true;
                done = false;
                break;
            }
            *choice = false;
        }

        if done {
            break;
        }
    }

    count
}

fn task2(input: &Input, target: u32) -> u32 {
    let mut choices: Vec<_> = input.iter().map(|_| false).collect();

    let mut count = 0;
    let mut minimum_used = usize::MAX;
    loop {
        let sum = input.iter().enumerate().fold(
            0,
            |acc, (index, size)| if choices[index] { acc + size } else { acc },
        );

        if sum == target {
            let used = choices.iter().filter(|c| **c).count();
            if used < minimum_used {
                count = 0;
                minimum_used = used;
            }
            if used <= minimum_used {
                count += 1;
            }
        }

        let mut done = true;
        for choice in choices.iter_mut() {
            if *choice == false {
                *choice = true;
                done = false;
                break;
            }
            *choice = false;
        }

        if done {
            break;
        }
    }

    count
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input, 150));
    println!("Task 2: {}", task2(&input, 150));
}

#[test]
fn example() {
    let test_input = "
        20
        15
        10
        5
        5";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input, 25), 4);
    assert_eq!(task2(&input, 25), 3);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input, 150), 1304);
    assert_eq!(task2(&input, 150), 18);
}
