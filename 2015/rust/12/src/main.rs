use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<char>;

fn parse_input(input: &str) -> Input {
    input.trim().chars().collect()
}

fn task1(input: &Input) -> u32 {
    let mut neg = false;
    let mut acc = 0;
    let mut current = 0;
    for c in input {
        if *c as u8 >= '0' as u8 && *c as u8 <= '9' as u8 {
            current = current * 10 + *c as u8 - '0' as u8;
        } else {
            if neg {
                acc -= current as u32;
            } else {
                acc += current as u32;
            }
            current = 0;
            neg = *c == '-';
        }
    }
    acc
}

fn parse_object(input: &Input, index: &mut usize) -> u32 {
    let mut neg = false;
    let mut acc = 0;
    let mut current = 0;

    let mut array_depth = 0;
    let mut has_red = false;
    let mut red = 0;

    while *index < input.len() {
        let c = input[*index];

        red = match red {
            1 if c == 'r' => 2,
            2 if c == 'e' => 3,
            3 if c == 'd' => 4,
            4 if c == '"' => {
                if array_depth == 0 {
                    has_red = true;
                }
                0
            }
            _ if c == '"' => 1,
            _ => 0,
        };

        if c as u8 >= '0' as u8 && c as u8 <= '9' as u8 {
            current = current * 10 + c as u8 - '0' as u8;
        } else {
            if neg {
                acc -= current as u32;
            } else {
                acc += current as u32;
            }
            current = 0;
            neg = c == '-';

            if c == '[' {
                array_depth += 1;
            };
            if c == ']' {
                array_depth -= 1;
            };
            if c == '}' {
                return if has_red { 0 } else { acc };
            }
            if c == '{' {
                *index += 1;
                acc += parse_object(input, index);
            }
        }

        *index += 1;
    }

    if has_red {
        0
    } else {
        acc
    }
}

fn task2(input: &Input) -> u32 {
    let mut index = 0;
    parse_object(input, &mut index)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "[1,2,3]";
    let test_input2 = "{\"a\":2,\"b\":4}";
    let test_input3 = "[[[3]]]";
    let test_input4 = "{\"a\":{\"b\":4},\"c\":-1}";
    let test_input5 = "{\"a\":[-1,1]}";
    let test_input6 = "[-1,{\"a\":1}]";
    let test_input7 = "[]";
    let test_input8 = "{}";

    let input1 = parse_input(&test_input1);
    let input2 = parse_input(&test_input2);
    let input3 = parse_input(&test_input3);
    let input4 = parse_input(&test_input4);
    let input5 = parse_input(&test_input5);
    let input6 = parse_input(&test_input6);
    let input7 = parse_input(&test_input7);
    let input8 = parse_input(&test_input8);
    assert_eq!(task1(&input1), 6);
    assert_eq!(task1(&input2), 6);
    assert_eq!(task1(&input3), 3);
    assert_eq!(task1(&input4), 3);
    assert_eq!(task1(&input5), 0);
    assert_eq!(task1(&input6), 0);
    assert_eq!(task1(&input7), 0);
    assert_eq!(task1(&input8), 0);

    let test_input1 = "[1,2,3]";
    let test_input2 = "[1,{\"c\":\"red\",\"b\":2},3]";
    let test_input3 = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
    let test_input4 = r#"[1,"red",5]"#;

    let input1 = parse_input(&test_input1);
    let input2 = parse_input(&test_input2);
    let input3 = parse_input(&test_input3);
    let input4 = parse_input(&test_input4);

    assert_eq!(task2(&input1), 6);
    assert_eq!(task2(&input2), 4);
    assert_eq!(task2(&input3), 0);
    assert_eq!(task2(&input4), 6);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 191164);
    assert_eq!(task2(&input), 87842);
}
