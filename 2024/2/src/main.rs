use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<i32>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_report(report: &Vec<i32>) -> bool {
    let is_increasing = report.windows(2).all(|window| window[0] < window[1]);
    let is_decreasing = report.windows(2).all(|window| window[0] > window[1]);
    let has_valid_differences = report.windows(2).all(|window| {
        let difference = (window[1] - window[0]).abs();
        difference >= 1 && difference <= 3
    });
    (is_increasing || is_decreasing) && has_valid_differences
}

fn task1(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| is_valid_report(report))
        .count()
}

fn task2(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| {
            if is_valid_report(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut combined = report[..i].to_vec();
                combined.extend(report[(i + 1)..].to_vec());
                if is_valid_report(&combined) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 2);
    assert_eq!(task2(&input), 4);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 257);
    assert_eq!(task2(&input), 328);
}
