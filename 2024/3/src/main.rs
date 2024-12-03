use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    input.trim().to_string()
}

fn task1(input: &Input) -> u32 {
    input
        .split("mul")
        .skip(1)
        .filter_map(|entry| {
            if entry.chars().nth(0) != Some('(') {
                return None;
            }
            if let Some((lhs, _)) = entry[1..].split_once(")") {
                if let Some((first, second)) = lhs.split_once(",") {
                    if (1..=3).contains(&first.len()) && (1..=3).contains(&second.len()) {
                        if first.chars().all(|char| char.is_digit(10))
                            && second.chars().all(|char| char.is_digit(10))
                        {
                            return Some((
                                first.parse::<u32>().unwrap(),
                                second.parse::<u32>().unwrap(),
                            ));
                        }
                    }
                }
            }

            None
        })
        .fold(0, |acc, (first, second)| acc + first * second)
}

fn task2(input: &Input) -> u32 {
    let a = input
        .split("don't()")
        .enumerate()
        .filter_map(|(i, part)| {
            if i == 0 {
                return Some(part);
            }

            if let Some((_, rhs)) = part.split_once("do()") {
                return Some(rhs);
            }

            None
        })
        .collect::<Vec<_>>()
        .concat();

    task1(&a)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
    xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let input1 = parse_input(test_input1);

    let test_input2 = "
    xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let input2 = parse_input(test_input2);

    assert_eq!(task1(&input1), 161);
    assert_eq!(task2(&input2), 48);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 183788984);
    assert_eq!(task2(&input), 62098619);
}
