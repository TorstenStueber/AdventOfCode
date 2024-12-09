use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn task1(input: &Input) -> usize {
    let mut chars = HashMap::<char, Vec<(i32, i32)>>::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.iter().enumerate() {
        height = usize::max(height, y);
        for (x, char) in line.iter().enumerate() {
            width = usize::max(width, x);
            if *char != '.' {
                chars
                    .entry(*char)
                    .and_modify(|list| list.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_, positions) in chars {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let antenna1 = positions[i];
                let antenna2 = positions[j];
                let antinode = (2 * antenna2.0 - antenna1.0, 2 * antenna2.1 - antenna1.1);

                if (0..=width as i32).contains(&antinode.0)
                    && (0..=height as i32).contains(&antinode.1)
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn calculate_gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn task2(input: &Input) -> usize {
    let mut chars = HashMap::<char, Vec<(i32, i32)>>::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.iter().enumerate() {
        height = usize::max(height, y);
        for (x, char) in line.iter().enumerate() {
            width = usize::max(width, x);
            if *char != '.' {
                chars
                    .entry(*char)
                    .and_modify(|list| list.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
    }

    let mut antinodes = HashSet::new();

    for (_, positions) in chars {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let antenna1 = positions[i];
                let antenna2 = positions[j];

                let gcd = calculate_gcd(
                    (antenna2.0 - antenna1.0).abs(),
                    (antenna2.1 - antenna1.1).abs(),
                );
                let diff_x = (antenna2.0 - antenna1.0) / gcd;
                let diff_y = (antenna2.1 - antenna1.1) / gcd;

                let mut iterations = 1;
                loop {
                    let antinode = (
                        antenna1.0 + diff_x * iterations,
                        antenna1.1 + diff_y * iterations,
                    );

                    if (0..=width as i32).contains(&antinode.0)
                        && (0..=height as i32).contains(&antinode.1)
                    {
                        antinodes.insert(antinode);
                    } else {
                        break;
                    }

                    iterations += 1;
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 14);
    assert_eq!(task2(&input), 34);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 327);
    assert_eq!(task2(&input), 1233);
}
