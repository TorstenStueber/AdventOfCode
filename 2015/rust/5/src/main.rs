use std::collections::HashMap;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<String>;

fn parse_input(input: &str) -> Input {
    input.trim().split("\n").map(String::from).collect()
}

fn is_nice_1(string: &&String) -> bool {
    let rule1 = string
        .chars()
        .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
        .count()
        >= 3;

    let rule2 = string
        .as_bytes()
        .windows(2)
        .any(|elements| elements[0] == elements[1]);

    let rule3 = !string.as_bytes().windows(2).any(|elements| {
        elements == [b'a', b'b']
            || elements == [b'c', b'd']
            || elements == [b'p', b'q']
            || elements == [b'x', b'y']
    });

    rule1 && rule2 && rule3
}

fn is_nice_2(string: &&String) -> bool {
    let mut positions = HashMap::new();
    let mut rule1 = false;

    string
        .as_bytes()
        .windows(2)
        .enumerate()
        .for_each(|(position, elements)| {
            if let Some(previous_position) = positions.get(elements) {
                if *previous_position < position - 1 {
                    rule1 = true;
                }
            } else {
                positions.insert(elements, position);
            }
        });

    let rule2 = string
        .as_bytes()
        .windows(3)
        .any(|elements| elements[0] == elements[2]);

    rule1 && rule2
}

fn task1(input: &Input) -> usize {
    input.into_iter().filter(is_nice_1).count()
}

fn task2(input: &Input) -> usize {
    input.into_iter().filter(is_nice_2).count()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = String::from("ugknbfddgicrmopn");
    let test_input2 = String::from("aaa");
    let test_input3 = String::from("jchzalrnumimnmhp");
    let test_input4 = String::from("haegwjzuvuyypxyu");
    let test_input5 = String::from("dvszwmarrgswjxmb");

    assert_eq!(is_nice_1(&&test_input1), true);
    assert_eq!(is_nice_1(&&test_input2), true);
    assert_eq!(is_nice_1(&&test_input3), false);
    assert_eq!(is_nice_1(&&test_input4), false);
    assert_eq!(is_nice_1(&&test_input5), false);

    let test_input1 = String::from("qjhvhtzxzqqjkmpb");
    let test_input2 = String::from("xxyxx");
    let test_input3 = String::from("uurcxstgmygtbstg");
    let test_input4 = String::from("ieodomkazucvgmuy");

    assert_eq!(is_nice_2(&&test_input1), true);
    assert_eq!(is_nice_2(&&test_input2), true);
    assert_eq!(is_nice_2(&&test_input3), false);
    assert_eq!(is_nice_2(&&test_input4), false);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 258);
    assert_eq!(task2(&input), 53);
}
