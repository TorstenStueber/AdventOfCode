use std::{fs, ops::RangeInclusive};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Input {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

fn parse_input(input: String) -> Input {
    let input = input.split_once(": ").unwrap().1.trim();
    let (x, y) = input.split_once(", ").unwrap();
    let x = x.split_once("=").unwrap().1;
    let y = y.split_once("=").unwrap().1;

    let x = x.split_once("..").unwrap();
    let y = y.split_once("..").unwrap();

    Input {
        x: x.0.parse().unwrap()..=x.1.parse().unwrap(),
        y: y.0.parse().unwrap()..=y.1.parse().unwrap(),
    }
}

fn task1(input: &Input) -> i64 {
    let n = input.y.start().abs();
    n * (n - 1) / 2
}

fn task2(input: &Input) -> u64 {
    let mut count = 0;

    for original_x_v in 1..=*input.x.end() {
        for original_y_v in *input.y.start()..input.y.start().abs() {
            let mut x_v = original_x_v;
            let mut y_v = original_y_v;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += x_v;
                y += y_v;

                if x_v > 0 {
                    x_v -= 1;
                }
                y_v -= 1;

                if input.x.contains(&x) && input.y.contains(&y) {
                    count += 1;
                    break;
                }
                if x > *input.x.end() || y < *input.y.start() {
                    break;
                }
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
    let input = parse_input(String::from("target area: x=20..30, y=-10..-5"));

    assert_eq!(task1(&input), 45);
    assert_eq!(task2(&input), 112);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 8911);
    assert_eq!(task2(&input), 4748);
}
