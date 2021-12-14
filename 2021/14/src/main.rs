use std::collections::HashMap;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Input {
    rules: HashMap<(char, char), char>,
    template: String,
}

fn parse_input(input: String) -> Input {
    let (template, rules_string) = input.trim().split_once("\n").unwrap();

    let mut rules = HashMap::new();

    rules_string.trim().lines().for_each(|line| {
        let (from, to) = line.split_once("->").unwrap();
        let mut from = from.trim().chars();
        let mut to = to.trim().chars();
        let (a, b) = (from.next().unwrap(), from.next().unwrap());
        rules.insert((a, b), to.next().unwrap());
    });

    Input {
        rules,
        template: String::from(template.trim()),
    }
}

fn step(input: &Input, pairs: HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let mut result = HashMap::new();

    for ((a, b), count) in pairs.iter() {
        let c = input.rules.get(&(*a, *b)).unwrap();
        *result.entry((*a, *c)).or_insert(0) += count;
        *result.entry((*c, *b)).or_insert(0) += count;
    }

    result
}

fn steps(input: &Input, times: i32) -> i64 {
    let mut pairs = HashMap::new();
    let template: Vec<_> = input.template.chars().collect();
    template.windows(2).for_each(|window| {
        *pairs.entry((window[0], window[1])).or_insert(0) += 1;
    });

    for _ in 0..times {
        pairs = step(input, pairs);
    }

    let mut counts = HashMap::new();
    *counts.entry(template[0]).or_insert(0) += 1;
    *counts.entry(template[template.len() - 1]).or_insert(0) += 1;

    for ((a, b), count) in pairs {
        *counts.entry(a).or_insert(0) += count;
        *counts.entry(b).or_insert(0) += count;
    }

    let counts: Vec<_> = counts.iter().map(|(_, count)| count / 2).collect();
    let min = counts.iter().min().unwrap();
    let max = counts.iter().max().unwrap();

    *max - *min
}

fn task1(input: &Input) -> i64 {
    steps(input, 10)
}

fn task2(input: &Input) -> i64 {
    steps(input, 40)
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C

        ",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 1588);
    assert_eq!(task2(&input), 2188189693529);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 3118);
    assert_eq!(task2(&input), 4332887448171);
}
