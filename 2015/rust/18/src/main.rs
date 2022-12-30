use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().map(|c| c == '#').collect())
        .collect()
}

fn iterate(input: Input) -> Input {
    let width = input[0].len() as i32;
    let height = input.len() as i32;

    input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, c)| {
                    let mut count = 0;
                    for xd in -1..=1 {
                        for yd in -1..=1 {
                            if xd == 0 && yd == 0 {
                                continue;
                            }
                            let xx = x as i32 + xd;
                            let yy = y as i32 + yd;
                            if xx < 0 || xx >= width || yy < 0 || yy >= height {
                                continue;
                            }
                            if input[yy as usize][xx as usize] {
                                count += 1;
                            }
                        }
                    }
                    if *c {
                        count == 2 || count == 3
                    } else {
                        count == 3
                    }
                })
                .collect()
        })
        .collect()
}

fn task1(input: &Input, steps: u32) -> u32 {
    let mut input = input.clone();
    for _ in 0..steps {
        input = iterate(input);
    }

    input.iter().fold(0, |acc, line| {
        acc + line.iter().fold(0, |acc, c| acc + if *c { 1 } else { 0 })
    })
}

fn task2(input: &Input, steps: u32) -> u32 {
    let mut input = input.clone();
    let width = input[0].len();
    let height = input.len();

    for _ in 0..steps {
        input[0][0] = true;
        input[0][width - 1] = true;
        input[height - 1][0] = true;
        input[height - 1][width - 1] = true;
        input = iterate(input);
    }

    input[0][0] = true;
    input[0][width - 1] = true;
    input[height - 1][0] = true;
    input[height - 1][width - 1] = true;

    input.iter().fold(0, |acc, line| {
        acc + line.iter().fold(0, |acc, c| acc + if *c { 1 } else { 0 })
    })
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input, 100));
    println!("Task 2: {}", task2(&input, 100));
}

#[test]
fn example() {
    let test_input = "
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input, 4), 4);
    assert_eq!(task2(&input, 5), 17);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input, 100), 821);
    assert_eq!(task2(&input, 100), 886);
}
