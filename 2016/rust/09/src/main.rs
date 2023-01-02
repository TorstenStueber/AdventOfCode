use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    input.trim().into()
}

fn task1(input: &Input) -> usize {
    let input: Vec<_> = input.chars().collect();

    let mut result = vec![];
    let mut index = 0;
    while index < input.len() {
        if input[index] == '(' {
            index += 1;
            let mut count1 = 0;
            let mut count2 = 0;
            while input[index] != 'x' {
                count1 = count1 * 10 + input[index] as usize - '0' as usize;
                index += 1;
            }
            index += 1;
            while input[index] != ')' {
                count2 = count2 * 10 + input[index] as usize - '0' as usize;
                index += 1;
            }
            index += 1;

            while count2 > 0 {
                for i in 0..count1 {
                    result.push(input[index + i])
                }
                count2 -= 1;
            }
            index += count1;
        } else {
            result.push(input[index]);
            index += 1;
        }
    }

    result.len()
}

fn recurse(input: &[char]) -> usize {
    let mut size = 0;
    let mut index = 0;
    while index < input.len() {
        if input[index] == '(' {
            index += 1;
            let mut count1 = 0;
            let mut count2 = 0;
            while input[index] != 'x' {
                count1 = count1 * 10 + input[index] as usize - '0' as usize;
                index += 1;
            }
            index += 1;
            while input[index] != ')' {
                count2 = count2 * 10 + input[index] as usize - '0' as usize;
                index += 1;
            }
            index += 1;
            size += count2 * recurse(&input[index..index + count1]);
            index += count1;
        } else {
            size += 1;
            index += 1;
        }
    }

    size
}

fn task2(input: &Input) -> usize {
    let input: Vec<_> = input.chars().collect();

    recurse(&input[..])
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    assert_eq!(task1(&parse_input("ADVENT")), 6);
    assert_eq!(task1(&parse_input("A(1x5)BC")), 7);
    assert_eq!(task1(&parse_input("(3x3)XYZ")), 9);
    assert_eq!(task1(&parse_input("A(2x2)BCD(2x2)EFG")), 11);
    assert_eq!(task1(&parse_input("(6x1)(1x3)A")), 6);
    assert_eq!(task1(&parse_input("X(8x2)(3x3)ABCY")), 18);

    assert_eq!(task2(&parse_input("(3x3)XYZ")), 9);
    assert_eq!(task2(&parse_input("X(8x2)(3x3)ABCY")), 20);
    assert_eq!(
        task2(&parse_input("(27x12)(20x12)(13x14)(7x10)(1x12)A")),
        241920
    );
    assert_eq!(
        task2(&parse_input(
            "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
        )),
        445
    );
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 99145);
    assert_eq!(task2(&input), 10943094568);
}
