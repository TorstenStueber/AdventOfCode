use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: String) -> Vec<Vec<u8>> {
    input
        .trim()
        .split("\n")
        .map(|line| Vec::from(line.trim().as_bytes()))
        .collect()
}

fn task1(input: &Vec<Vec<u8>>) -> u64 {
    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;

    let length = input[0].len();
    for i in 0..length {
        let mut count_0 = 0;
        for line in input {
            if line[i] == b'0' {
                count_0 += 1;
            }
        }

        let bit = if count_0 * 2 > input.len() { 0 } else { 1 };
        gamma = 2 * gamma + bit;
        epsilon = 2 * epsilon + 1 - bit;
    }

    gamma * epsilon
}

fn determine_oxygen_bit(count_0: usize, length: usize) -> u8 {
    if count_0 * 2 > length {
        b'0'
    } else {
        b'1'
    }
}

fn determine_co2_bit(count_0: usize, length: usize) -> u8 {
    if count_0 * 2 > length {
        b'1'
    } else {
        b'0'
    }
}

fn filter<T: Fn(usize, usize) -> u8>(
    input: Vec<Vec<u8>>,
    position: usize,
    determine_bit: T,
) -> Vec<Vec<u8>> {
    let mut count_0 = 0;
    for line in input.iter() {
        if line[position] == b'0' {
            count_0 += 1;
        }
    }

    let bit = determine_bit(count_0, input.len());

    input
        .into_iter()
        .filter(|line| line[position] == bit)
        .collect()
}

fn find<T: Fn(usize, usize) -> u8>(mut input: Vec<Vec<u8>>, determine_bit: &T) -> u64 {
    let mut i = 0;
    while input.len() > 1 {
        input = filter(input, i, determine_bit);
        i += 1;
    }

    let result = &input[0];
    let mut binary: u64 = 0;
    for bit in result {
        binary = 2 * binary + (if *bit == b'0' { 0 } else { 1 });
    }

    binary
}

fn task2(input: &Vec<Vec<u8>>) -> u64 {
    find(input.clone(), &determine_oxygen_bit) * find(input.clone(), &determine_co2_bit)
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 198);
    assert_eq!(task2(&input), 230);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 3242606);
    assert_eq!(task2(&input), 4856080);
}
