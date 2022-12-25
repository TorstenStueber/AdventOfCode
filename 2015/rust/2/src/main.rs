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
            let mut parsed_line = line.trim().split("x");
            (
                parsed_line.next().unwrap().parse().unwrap(),
                parsed_line.next().unwrap().parse().unwrap(),
                parsed_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn task1(input: &Input) -> u32 {
    input.iter().fold(0, |acc, (w, l, h)| {
        let slack = (w * l).min(w * h).min(l * h);
        acc + slack + 2 * (w * l + w * h + l * h)
    })
}

fn task2(input: &Input) -> u32 {
    input.iter().fold(0, |acc, (w, l, h)| {
        let perimeter = 2 * (w + l).min(w + h).min(l + h);
        acc + perimeter + (w * l * h)
    })
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
    2x3x4
    1x1x10";

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 58 + 43);
    assert_eq!(task2(&input), 34 + 14);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1586300);
    assert_eq!(task2(&input), 3737498);
}
