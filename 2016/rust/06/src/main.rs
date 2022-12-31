use std::collections::HashMap;
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

fn task1(input: &Input) -> String {
    let mut string = String::new();
    for index in 0..input[0].len() {
        let mut counts = HashMap::new();

        for line in input {
            let c = line.as_bytes()[index] as char;
            counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by_key(|(_, i)| -*i);
        string.push(counts[0].0);
    }

    string
}

fn task2(input: &Input) -> String {
    let mut string = String::new();
    for index in 0..input[0].len() {
        let mut counts = HashMap::new();

        for line in input {
            let c = line.as_bytes()[index] as char;
            counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut counts: Vec<_> = counts.into_iter().collect();
        counts.sort_by_key(|(_, i)| *i);
        string.push(counts[0].0);
    }

    string
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 1: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), "easter");
    assert_eq!(task2(&input), "advent");
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), "tkspfjcc");
    assert_eq!(task2(&input), "xrlmbypn");
}
