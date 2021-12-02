use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let parsed_line: Vec<&str> = line.split_whitespace().collect();
            let command = parsed_line[0];
            let number: u32 = parsed_line[1].parse().unwrap();
            match command {
                "forward" => Command::Forward(number),
                "up" => Command::Up(number),
                "down" => Command::Down(number),
                _ => panic!("Unknown command {}", command),
            }
        })
        .collect()
}

fn task1(input: &Vec<Command>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    for command in input {
        match command {
            Command::Forward(d) => x += d,
            Command::Down(d) => y += d,
            Command::Up(d) => y -= d,
        }
    }

    x * y
}

fn task2(input: &Vec<Command>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim: u32 = 0;

    for command in input {
        match command {
            Command::Forward(d) => {
                x += d;
                y += aim * d;
            }
            Command::Down(d) => aim += d,
            Command::Up(d) => aim -= d,
        }
    }

    x * y
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 150);
    assert_eq!(task2(&input), 900);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1746616);
    assert_eq!(task2(&input), 1741971043);
}
