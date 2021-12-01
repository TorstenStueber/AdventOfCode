use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn count_increments(list: &[u32]) -> usize {
    list.windows(2).filter(|slice| slice[0] < slice[1]).count()
}

fn task1(input: &Vec<u32>) -> usize {
    count_increments(&input)
}

fn task2(input: &Vec<u32>) -> usize {
    let windows: Vec<_> = input
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect();

    count_increments(&windows)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 7);
    assert_eq!(task2(&input), 5);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1301);
    assert_eq!(task2(&input), 1346);
}
