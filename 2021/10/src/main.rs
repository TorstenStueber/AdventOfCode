use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<String>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| String::from(line.trim()))
        .collect()
}

fn corrupted(line: &String) -> Result<u64, u64> {
    let mut stack = vec![];

    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' => {
                if stack.pop() != Some('(') {
                    return Err(3);
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Err(57);
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Err(1197);
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Err(25137);
                }
            }
            _ => panic!(),
        }
    }

    let mut score = 0;
    while let Some(char) = stack.pop() {
        score = 5 * score
            + match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            }
    }
    Ok(score)
}

fn task1(input: &Input) -> u64 {
    input.iter().filter_map(|line| corrupted(line).err()).sum()
}

fn task2(input: &Input) -> u64 {
    let mut result: Vec<_> = input
        .iter()
        .filter_map(|line| corrupted(line).ok())
        .collect();

    result.sort();
    result[result.len() / 2]
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
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        ",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 26397);
    assert_eq!(task2(&input), 288957);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 411471);
    assert_eq!(task2(&input), 3122628974);
}
