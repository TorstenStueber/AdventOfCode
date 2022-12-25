use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(String, String, u32)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (lhs, rhs) = line.split_once("=").unwrap();
            let (start, end) = lhs.split_once("to").unwrap();
            (
                start.trim().into(),
                end.trim().into(),
                rhs.trim().parse().unwrap(),
            )
        })
        .collect()
}

fn iterate_min(
    distances: &HashMap<(&String, &String), u32>,
    current: &String,
    remaining: Vec<&String>,
    current_distance: u32,
    minimum: &mut u32,
) {
    if remaining.len() == 0 {
        *minimum = (*minimum).min(current_distance);
        return;
    }

    for (position, town) in remaining.iter().enumerate() {
        let mut remaining = remaining.clone();
        remaining.remove(position);
        let current_distance = current_distance + distances.get(&(current, town)).unwrap();
        iterate_min(distances, town, remaining, current_distance, minimum);
    }
}

fn task1(input: &Input) -> u32 {
    let mut towns = HashSet::new();
    let mut distances = HashMap::new();
    for (town1, town2, distance) in input {
        towns.insert(town1);
        towns.insert(town2);
        distances.insert((town1, town2), *distance);
        distances.insert((town2, town1), *distance);
    }

    let towns: Vec<_> = towns.into_iter().collect();

    let mut minimum = u32::MAX;
    for (position, first_town) in towns.iter().enumerate() {
        let mut remaining = towns.clone();
        remaining.remove(position);
        iterate_min(&distances, first_town, remaining, 0, &mut minimum);
    }

    minimum
}

fn iterate_max(
    distances: &HashMap<(&String, &String), u32>,
    current: &String,
    remaining: Vec<&String>,
    current_distance: u32,
    maximum: &mut u32,
) {
    if remaining.len() == 0 {
        *maximum = (*maximum).max(current_distance);
        return;
    }

    for (position, town) in remaining.iter().enumerate() {
        let mut remaining = remaining.clone();
        remaining.remove(position);
        let current_distance = current_distance + distances.get(&(current, town)).unwrap();
        iterate_max(distances, town, remaining, current_distance, maximum);
    }
}

fn task2(input: &Input) -> u32 {
    let mut towns = HashSet::new();
    let mut distances = HashMap::new();
    for (town1, town2, distance) in input {
        towns.insert(town1);
        towns.insert(town2);
        distances.insert((town1, town2), *distance);
        distances.insert((town2, town1), *distance);
    }

    let towns: Vec<_> = towns.into_iter().collect();

    let mut maximum = u32::MIN;
    for (position, first_town) in towns.iter().enumerate() {
        let mut remaining = towns.clone();
        remaining.remove(position);
        iterate_max(&distances, first_town, remaining, 0, &mut maximum);
    }

    maximum
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

    let input = parse_input(&test_input);

    assert_eq!(task1(&input), 605);
    assert_eq!(task2(&input), 982);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 207);
    assert_eq!(task2(&input), 804);
}
