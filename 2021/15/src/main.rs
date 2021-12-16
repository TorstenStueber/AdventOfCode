use std::collections::{HashMap, HashSet};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<u32>>;

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

    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(xd, yd)| {
            let xx = x as i32 + xd;
            let yy = y as i32 + yd;

            if xx >= 0 && xx < width && yy >= 0 && yy < height {
                Some((xx as usize, yy as usize))
            } else {
                None
            }
        })
        .collect()
}

fn solve(input: &Input) -> u32 {
    let mut candidates = HashMap::new();
    candidates.insert(
        (input[0].len() - 1, input.len() - 1),
        *input.last().unwrap().last().unwrap(),
    );

    let mut done = HashSet::new();

    loop {
        let mut hash_iter = candidates.iter();
        let mut best = hash_iter.next().unwrap();
        for current in hash_iter {
            if *current.1 < *best.1 {
                best = current;
            }
        }

        let (&position, &distance) = best;
        let (x, y) = position;
        if x == 0 && y == 0 {
            return distance - input[0][0];
        }

        for (xx, yy) in neighbors(input, x, y) {
            if done.contains(&(xx, yy)) {
                continue;
            }

            let dd = distance + input[yy][xx];

            let entry = candidates.entry((xx, yy)).or_insert(dd);
            *entry = (*entry).min(dd);
        }

        candidates.remove(&(x, y));
        done.insert((x, y));
    }
}

fn task1(input: &Input) -> u32 {
    solve(input)
}

fn task2(input: &Input) -> u32 {
    let mut scaled_input = vec![];

    let width = input[0].len();
    let height = input.len();
    for y in 0..height * 5 {
        let mut line = vec![];
        for x in 0..width * 5 {
            let add = x / width + y / height;
            let value = add as u32 + input[y % height][x % width];

            line.push(if value > 9 { value - 9 } else { value });
        }
        scaled_input.push(line);
    }
    solve(&scaled_input)
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
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        ",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 40);
    assert_eq!(task2(&input), 315);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 410);
    assert_eq!(task2(&input), 2809);
}
