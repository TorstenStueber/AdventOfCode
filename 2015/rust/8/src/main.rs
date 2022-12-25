use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<String>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().into())
        .collect()
}

fn task1(input: &Input) -> usize {
    input.iter().fold(0, |acc, line| {
        let line = line.as_bytes();
        let mut index = 0;
        let mut counter = 0;
        while index < line.len() {
            if line[index] == b'\\' && index < line.len() - 1 {
                if line[index + 1] == b'"' || line[index + 1] == b'\\' {
                    index += 1;
                    counter += 1;
                } else if line[index + 1] == b'x' && index < line.len() - 3 {
                    index += 3;
                    counter += 3;
                }
            }
            index += 1;
        }

        acc + 2 + counter
    })
}

fn task2(input: &Input) -> usize {
    input.iter().fold(0, |acc, line| {
        let mut counter = 0;
        for byte in line.as_bytes() {
            if *byte == b'\\' || *byte == b'"' {
                counter += 1;
            }
        }
        acc + counter + 2
    })
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
        "#;

    let input = parse_input(&test_input);

    assert_eq!(task1(&input), 12);
    assert_eq!(task2(&input), 19);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1342);
    assert_eq!(task2(&input), 2074);
}
