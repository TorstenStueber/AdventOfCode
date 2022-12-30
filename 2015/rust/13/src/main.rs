use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(String, String, i32)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim();
            let line = &line[..line.len() - 1];
            let (lhs, rhs) = line
                .split_once("happiness units by sitting next to")
                .unwrap();
            if let Some((name, points)) = lhs.split_once("would gain") {
                return (
                    name.trim().into(),
                    rhs.trim().into(),
                    points.trim().parse().unwrap(),
                );
            } else if let Some((name, points)) = lhs.split_once("would lose") {
                return (
                    name.trim().into(),
                    rhs.trim().into(),
                    -(points.trim().parse::<i32>().unwrap()),
                );
            } else {
                panic!()
            }
        })
        .collect()
}

fn recurse(
    first_name: &String,
    map: &HashMap<(&String, &String), i32>,
    current_name: &String,
    remaining: Vec<&String>,
    current_points: i32,
    maximum: &mut i32,
) {
    if remaining.len() == 0 {
        let final_points = current_points
            + map.get(&(current_name, first_name)).unwrap()
            + map.get(&(first_name, current_name)).unwrap();
        *maximum = final_points.max(*maximum);
        return;
    }

    for index in 0..remaining.len() {
        let mut remaining = remaining.clone();
        let next_name = remaining.remove(index);

        let next_points = current_points
            + map.get(&(current_name, next_name)).unwrap()
            + map.get(&(next_name, current_name)).unwrap();

        recurse(first_name, map, next_name, remaining, next_points, maximum);
    }
}

fn task1(input: &Input) -> i32 {
    let mut map = HashMap::new();
    let mut names = HashSet::new();

    for (name1, name2, points) in input {
        names.insert(name1);
        names.insert(name2);
        map.insert((name1, name2), *points);
    }

    let mut names: Vec<_> = names.into_iter().collect();
    let first_name = names.remove(0);

    let mut maximum = i32::MIN;
    recurse(first_name, &map, first_name, names, 0, &mut maximum);

    maximum
}

fn task2(input: &Input) -> i32 {
    let mut map = HashMap::new();
    let mut names = HashSet::new();

    for (name1, name2, points) in input {
        names.insert(name1);
        names.insert(name2);
        map.insert((name1, name2), *points);
    }

    let me = String::from("*");
    for name in names.iter() {
        map.insert((name, &me), 0);
        map.insert((&me, name), 0);
    }
    names.insert(&me);

    let mut names: Vec<_> = names.into_iter().collect();
    let first_name = names.remove(0);

    let mut maximum = i32::MIN;
    recurse(first_name, &map, first_name, names, 0, &mut maximum);

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
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input), 330);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 664);
    assert_eq!(task2(&input), 640);
}
