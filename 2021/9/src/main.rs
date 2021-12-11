use std::collections::HashSet;
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

fn low_points(input: &Input) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (y, row) in input.iter().enumerate() {
        for (x, level) in row.iter().enumerate() {
            if neighbors(input, x, y)
                .iter()
                .all(|(xx, yy)| input[*yy][*xx] > *level)
            {
                result.push((x, y));
            }
        }
    }

    result
}

fn task1(input: &Input) -> i32 {
    low_points(input)
        .iter()
        .map(|(x, y)| 1 + input[*y][*x])
        .sum()
}

fn task2(input: &Input) -> usize {
    let mut sizes: Vec<_> = low_points(input)
        .iter()
        .map(|(x, y)| {
            let mut basin = HashSet::<(usize, usize)>::new();
            let mut todo = vec![(*x, *y)];

            while let Some((x, y)) = todo.pop() {
                let n = neighbors(input, x, y);
                let n: Vec<_> = n
                    .iter()
                    .filter(|(xx, yy)| !basin.contains(&(*xx, *yy)) && input[*yy][*xx] != 9)
                    .collect();

                n.iter().for_each(|(xx, yy)| {
                    basin.insert((*xx, *yy));
                    todo.push((*xx, *yy));
                });
            }

            basin.len()
        })
        .collect();

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
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
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 15);
    assert_eq!(task2(&input), 1134);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 580);
    assert_eq!(task2(&input), 856716);
}
