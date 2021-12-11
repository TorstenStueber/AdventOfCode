use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<i32>>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| String::from(char).parse().unwrap())
                .collect()
        })
        .collect()
}

fn neighbors(input: &Input, x: usize, y: usize) -> Vec<(usize, usize)> {
    let width = input[0].len() as i32;
    let height = input.len() as i32;

    let mut result = vec![];
    for xd in -1..=1 {
        for yd in -1..=1 {
            if xd == 0 && yd == 0 {
                continue;
            }

            let xx = x as i32 + xd;
            let yy = y as i32 + yd;

            if xx >= 0 && xx < width && yy >= 0 && yy < height {
                result.push((xx as usize, yy as usize))
            }
        }
    }

    result
}

fn step(input: &mut Input) -> u32 {
    let mut flashes = 0;
    let width = input[0].len();
    let height = input.len();

    for x in 0..width {
        for y in 0..height {
            input[x][y] += 1;
        }
    }

    loop {
        let mut done = true;
        for x in 0..width {
            for y in 0..height {
                if input[x][y] > 9 {
                    done = false;
                    flashes += 1;
                    input[x][y] = 0;
                    for (xx, yy) in neighbors(input, x, y).iter() {
                        if input[*xx][*yy] != 0 {
                            input[*xx][*yy] += 1
                        }
                    }
                }
            }
        }

        if done {
            break;
        }
    }

    flashes
}

fn task1(input: &Input) -> u32 {
    let mut input = input.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut input)
    }

    flashes
}

fn task2(input: &Input) -> u64 {
    let mut input = input.clone();

    for i in 1.. {
        if step(&mut input) == 100 {
            return i;
        }
    }

    unreachable!();
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
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 1656);
    assert_eq!(task2(&input), 195);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 1729);
    assert_eq!(task2(&input), 237);
}
