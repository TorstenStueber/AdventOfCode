use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

fn task1(input: &Vec<char>) -> i32 {
    input
        .iter()
        .fold(0, |acc, c| if *c == '(' { acc + 1 } else { acc - 1 })
}

fn task2(input: &Vec<char>) -> usize {
    let mut level = 0;
    for (position, c) in input.iter().enumerate() {
        level += if *c == '(' { 1 } else { -1 };
        if level == -1 {
            return position + 1;
        }
    }

    unreachable!()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = parse_input("(())");
    let test_input2 = parse_input("()()");
    let test_input3 = parse_input("(((");
    let test_input4 = parse_input("(()(()(");
    let test_input5 = parse_input("))(((((");
    let test_input6 = parse_input("())");
    let test_input7 = parse_input("))(");
    let test_input8 = parse_input(")))");
    let test_input9 = parse_input(")())())");

    assert_eq!(task1(&test_input1), 0);
    assert_eq!(task1(&test_input2), 0);
    assert_eq!(task1(&test_input3), 3);
    assert_eq!(task1(&test_input4), 3);
    assert_eq!(task1(&test_input5), 3);
    assert_eq!(task1(&test_input6), -1);
    assert_eq!(task1(&test_input7), -1);
    assert_eq!(task1(&test_input8), -3);
    assert_eq!(task1(&test_input9), -3);

    let test_input1 = parse_input(")");
    let test_input2 = parse_input("()())");

    assert_eq!(task2(&test_input1), 1);
    assert_eq!(task2(&test_input2), 5);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 232);
    assert_eq!(task2(&input), 1783);
}
