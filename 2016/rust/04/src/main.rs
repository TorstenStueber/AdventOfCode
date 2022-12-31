use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(Vec<String>, u32, String)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (rhs, lhs) = line.trim().split_once("[").unwrap();
            let mut rhs: Vec<_> = rhs.split("-").map(String::from).collect();
            let id = rhs[rhs.len() - 1].parse().unwrap();
            rhs.pop();
            (rhs, id, lhs[..lhs.len() - 1].into())
        })
        .collect()
}

fn is_correct(room: &&(Vec<String>, u32, String)) -> bool {
    let (names, _, checksum) = room;

    let mut occurrences = HashMap::new();
    for name in names {
        for c in name.chars() {
            occurrences.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    let mut occurrences: Vec<_> = occurrences.into_iter().collect();
    occurrences.sort_by(|a, b| {
        if a.1 < b.1 {
            Ordering::Greater
        } else if a.1 > b.1 {
            Ordering::Less
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });

    checksum.chars().collect::<Vec<_>>()
        == occurrences
            .iter()
            .take(5)
            .map(|(c, _)| *c)
            .collect::<Vec<_>>()
}

fn task1(input: &Input) -> u32 {
    input.iter().filter(is_correct).map(|(_, id, _)| *id).sum()
}

fn task2(input: &Input) -> u32 {
    input
        .iter()
        .filter(is_correct)
        .map(|(name, id, _)| {
            let name = name
                .iter()
                .map(|word| {
                    String::from_iter(word.chars().map(|c| {
                        (((c as u8 - 'a' as u8) as u32 + id) % 26 + 'a' as u32) as u8 as char
                    }))
                })
                .collect::<Vec<_>>()
                .join(" ");
            (name, id)
        })
        .filter(|(name, _)| name == "northpole object storage")
        .map(|(_, id)| *id)
        .next()
        .unwrap()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 1: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        aaaaa-bbb-z-y-x-123[abxyz]
        a-b-c-d-e-f-g-h-987[abcde]
        not-a-real-room-404[oarel]
        totally-real-room-200[decoy]";
    let input = parse_input(input);

    assert_eq!(task1(&input), 1514);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 361724);
    assert_eq!(task2(&input), 482);
}
