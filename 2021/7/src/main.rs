use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<i32>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect()
}

fn task1(input: &Input) -> i32 {
    let mut input = input.clone();
    input.sort();

    let median = input[input.len() / 2];

    input.iter().fold(0, |a, b| a + (*b - median).abs())
}

fn task2(input: &Input) -> i32 {
    let mut shortest_distance = i32::MAX;

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    for position in min..=max {
        let current_distance = input.iter().fold(0, |a, b| {
            let distance = (*b - position).abs();
            a + distance * (distance + 1) / 2
        });

        if current_distance < shortest_distance {
            shortest_distance = current_distance;
        }
    }

    shortest_distance
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from("16,1,2,0,4,2,7,1,2,14");

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 37);
    assert_eq!(task2(&input), 168);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 364898);
    assert_eq!(task2(&input), 104149091);
}
