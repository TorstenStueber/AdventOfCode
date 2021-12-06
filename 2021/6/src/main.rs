use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = [u64; 9];

fn parse_input(input: String) -> Input {
    let mut counts = [0u64; 9];

    for number in input.trim().split(",") {
        counts[number.parse::<usize>().unwrap()] += 1;
    }

    counts
}

fn iterate(input: &Input, days: u32) -> u64 {
    let mut counts = input.clone();
    for _ in 0..days {
        let count0 = counts[0];
        for j in 0..8 {
            counts[j] = counts[j + 1];
        }

        counts[6] += count0;
        counts[8] = count0;
    }

    counts.iter().sum()
}

fn task1(input: &Input) -> u64 {
    iterate(input, 80)
}

fn task2(input: &Input) -> u64 {
    iterate(input, 256)
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from("3,4,3,1,2");

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 5934);
    assert_eq!(task2(&input), 26984457539);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 390923);
    assert_eq!(task2(&input), 1749945484935);
}
