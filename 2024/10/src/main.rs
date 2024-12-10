use std::{collections::HashSet, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<usize>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| {
                    if char == '.' {
                        usize::MAX
                    } else {
                        char.to_digit(10).unwrap() as usize
                    }
                })
                .collect()
        })
        .collect()
}

fn task1(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut start_points = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 0 {
                start_points.push((x, y));
            }
        }
    }

    let mut result = 0;
    for (x, y) in start_points {
        let mut current_points = HashSet::new();
        current_points.insert((x, y));

        for step in 1..=9 {
            let mut next_points = HashSet::new();
            for (x, y) in current_points {
                for (diffx, diffy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (next_x, next_y) =
                        ((x as i32 + diffx) as usize, (y as i32 + diffy) as usize);
                    if (0..width).contains(&next_x)
                        && (0..height).contains(&next_y)
                        && input[next_y][next_x] == step
                    {
                        next_points.insert((next_x, next_y));
                    }
                }
            }
            current_points = next_points;
            if step == 9 {
                result += current_points.len();
            }
        }
    }

    result
}

fn task2(input: &Input) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut start_points = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 0 {
                start_points.push((x, y));
            }
        }
    }

    let mut result = 0;
    for (x, y) in start_points {
        let mut current_points = Vec::new();
        current_points.push((x, y));

        for step in 1..=9 {
            let mut next_points = Vec::new();
            for (x, y) in current_points {
                for (diffx, diffy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (next_x, next_y) =
                        ((x as i32 + diffx) as usize, (y as i32 + diffy) as usize);
                    if (0..width).contains(&next_x)
                        && (0..height).contains(&next_y)
                        && input[next_y][next_x] == step
                    {
                        next_points.push((next_x, next_y));
                    }
                }
            }
            current_points = next_points;
            if step == 9 {
                result += current_points.len();
            }
        }
    }

    result
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        0123
        1234
        8765
        9876";

    let test_input2 = "
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9";

    let test_input3 = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....";

    let test_input4 = "
        10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01";

    let test_input5 = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732";

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);
    let input3 = parse_input(test_input3);
    let input4 = parse_input(test_input4);
    let input5 = parse_input(test_input5);

    assert_eq!(task1(&input1), 1);
    assert_eq!(task1(&input2), 2);
    assert_eq!(task1(&input3), 4);
    assert_eq!(task1(&input4), 3);
    assert_eq!(task1(&input5), 36);

    let test_input6 = "
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....";

    let test_input7 = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....";

    let test_input8 = "
        012345
        123456
        234567
        345678
        4.6789
        56789.";

    let input6 = parse_input(test_input6);
    let input7 = parse_input(test_input7);
    let input8 = parse_input(test_input8);

    assert_eq!(task2(&input6), 3);
    assert_eq!(task2(&input7), 13);
    assert_eq!(task2(&input8), 227);
    assert_eq!(task2(&input5), 81);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 717);
    assert_eq!(task2(&input), 1686);
}
