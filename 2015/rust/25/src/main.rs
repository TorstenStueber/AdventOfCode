use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = (u32, u32);

fn parse_input(input: &str) -> Input {
    let (row, column) = input
        .trim()
        .split_once("row")
        .unwrap()
        .1
        .split_once(", column")
        .unwrap();
    let column = column.split_once(".").unwrap().0;
    (row.trim().parse().unwrap(), column.trim().parse().unwrap())
}

fn task1(input: &Input) -> u64 {
    let (mut x, mut y) = (1, 1);
    let mut n = 20151125u64;
    while (x, y) != *input {
        (x, y) = if x == 1 { (y + 1, 1) } else { (x - 1, y + 1) };
        n = (n * 252533) % 33554393;
    }
    n
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
}

#[test]
fn example() {
    let input = (6, 2);
    assert_eq!(task1(&input), 6796745);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 9132360);
}
