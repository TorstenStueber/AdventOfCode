use std::fs;

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

fn task1(input: &Input) -> u32 {
    let term = b"XMAS";
    let mut result = 0;

    let height = input.len() as isize;
    let width = input[0].len() as isize;

    let directions = [
        (0isize, 1isize),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for y in 0..height {
        for x in 0..width {
            for (dy, dx) in directions {
                let endx = x + dx * (term.len() - 1) as isize;
                let endy = y + dy * (term.len() - 1) as isize;

                if (0..width).contains(&endx) && (0..height).contains(&endy) {
                    if (0..term.len() as isize).all(|i| {
                        input[(y + dy * i) as usize][(x + dx * i) as usize]
                            == term[i as usize] as char
                    }) {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

fn task2(input: &Input) -> u32 {
    let mut result = 0;

    let height = input.len() as isize;
    let width = input[0].len() as isize;

    let directions = [(1isize, 1isize), (1, -1), (-1, -1), (-1, 1)];

    let at = |x, y, c| {
        (0..width).contains(&x) && (0..height).contains(&y) && input[y as usize][x as usize] == c
    };

    for y in 0..height {
        for x in 0..width {
            for (dy, dx) in directions {
                let conditions = [
                    (x, y, 'A'),
                    (x + dx, y + dy, 'M'),
                    (x - dx, y - dy, 'S'),
                    (x + dy, y - dx, 'M'),
                    (x - dy, y + dx, 'S'),
                ];

                if conditions.iter().all(|(x, y, c)| at(*x, *y, *c)) {
                    result += 1;
                }
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
    let test_input = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 18);
    assert_eq!(task2(&input), 9);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 2583);
    assert_eq!(task2(&input), 1978);
}
