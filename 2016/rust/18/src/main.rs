use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

use Tile::*;

impl Tile {
    fn next(left: Self, center: Self, right: Self) -> Self {
        match (left, center, right) {
            (Trap, Trap, Safe) => Trap,
            (Safe, Trap, Trap) => Trap,
            (Trap, Safe, Safe) => Trap,
            (Safe, Safe, Trap) => Trap,
            _ => Safe,
        }
    }
}

type Input = Vec<Tile>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|c| if c == '.' { Safe } else { Trap })
        .collect()
}

fn execute(input: &Input, rows: u32) -> usize {
    let mut input = input.clone();

    let mut safe_count = input.iter().filter(|tile| **tile == Safe).count();

    for _ in 1..rows {
        input = input
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                Tile::next(
                    if index > 0 { input[index - 1] } else { Safe },
                    *tile,
                    if index < input.len() - 1 {
                        input[index + 1]
                    } else {
                        Safe
                    },
                )
            })
            .collect();

        safe_count += input.iter().filter(|tile| **tile == Safe).count();
    }

    safe_count
}

fn task1(input: &Input) -> usize {
    execute(input, 40)
}

fn task2(input: &Input) -> usize {
    execute(input, 400000)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input("..^^.");
    assert_eq!(execute(&input, 3), 6);

    let input = parse_input(".^^.^.^^^^");
    assert_eq!(execute(&input, 10), 38);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 2005);
    assert_eq!(task2(&input), 20008491);
}
