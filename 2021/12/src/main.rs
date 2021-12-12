use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(String, String)>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.trim().split_once("-").unwrap();
            (String::from(a), String::from(b))
        })
        .collect()
}

fn determine_node_map(input: &Input) -> HashMap<&str, Vec<&str>> {
    let mut node_map = HashMap::<&str, Vec<&str>>::new();
    for (a, b) in input {
        let a = a.as_str();
        let b = b.as_str();

        node_map.entry(a).or_insert(vec![]).push(b);
        node_map.entry(b).or_insert(vec![]).push(a);
    }

    node_map
}

fn task1(input: &Input) -> u32 {
    let node_map = determine_node_map(input);

    let mut todo = vec![("start", HashSet::<&str>::new())];

    let mut count = 0;
    while let Some((node, mut used)) = todo.pop() {
        if node == "end" {
            count += 1;
            continue;
        }

        if node.to_lowercase() == node {
            used.insert(node);
        }

        for &next in node_map.get(node).unwrap() {
            if !used.contains(next) {
                todo.push((next, used.clone()))
            }
        }
    }

    count
}

fn task2(input: &Input) -> u64 {
    let node_map = determine_node_map(input);

    let mut todo = vec![("start", HashSet::<&str>::new(), false)];

    let mut count = 0;
    while let Some((node, mut used, mut used_twice)) = todo.pop() {
        if node == "end" {
            count += 1;
            continue;
        }

        if node.to_lowercase() == node {
            if !used.contains(node) {
                used.insert(node);
            } else {
                used_twice = true;
            }
        }

        for &next in node_map.get(node).unwrap() {
            if !used.contains(next) || (!used_twice && next != "start" && next != "end") {
                todo.push((next, used.clone(), used_twice))
            }
        }
    }

    count
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = String::from(
        "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        ",
    );

    let test_input2 = String::from(
        "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
        ",
    );

    let test_input3 = String::from(
        "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        ",
    );

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);
    let input3 = parse_input(test_input3);
    assert_eq!(task1(&input1), 10);
    assert_eq!(task1(&input2), 19);
    assert_eq!(task1(&input3), 226);
    assert_eq!(task2(&input1), 36);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 3230);
    assert_eq!(task2(&input), 83475);
}
