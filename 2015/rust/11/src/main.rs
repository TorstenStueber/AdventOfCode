use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<char>;

fn parse_input(input: &str) -> Input {
    input.trim().chars().collect()
}

fn task1(input: &Input) -> String {
    let mut input = input.clone();

    loop {
        for c in input.iter_mut().rev() {
            if *c != 'z' {
                *c = ((*c as u8) + 1) as char;
                break;
            }
            *c = 'a';
        }

        if !input.windows(3).any(|window| {
            window[0] as u8 + 1 == window[1] as u8 && window[1] as u8 + 1 == window[2] as u8
        }) {
            continue;
        }

        if input.iter().any(|c| *c == 'i' || *c == 'o' || *c == 'l') {
            continue;
        }

        if let Some((index, _)) = input
            .windows(2)
            .enumerate()
            .find(|(_, window)| window[0] == window[1])
        {
            if !input
                .windows(2)
                .skip(index + 2)
                .any(|window| window[0] == window[1])
            {
                continue;
            }
        } else {
            continue;
        }

        return input.into_iter().collect();
    }
}

fn task2(input: &Input) -> String {
    task1(&task1(input).chars().collect())
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "abcdefgh";
    let test_input2 = "ghijklmn";

    let input1 = parse_input(&test_input1);
    let input2 = parse_input(&test_input2);

    assert_eq!(task1(&input1), String::from("abcdffaa"));
    assert_eq!(task1(&input2), String::from("ghjaabcc"));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), String::from("hxbxxyzz"));
    assert_eq!(task2(&input), String::from("hxcaabcc"));
}
