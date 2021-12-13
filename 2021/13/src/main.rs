use std::collections::HashSet;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug, Clone)]
struct Input {
    map: HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

fn parse_input(input: String) -> Input {
    let mut lines = input.trim().lines().map(|line| line.trim());

    let mut map = HashSet::<(i32, i32)>::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        map.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut folds = vec![];
    for fold in lines {
        let fold: Vec<_> = fold.split_whitespace().collect();
        let (dir, pos) = fold[2].split_once("=").unwrap();
        let pos = pos.parse().unwrap();

        if dir == "x" {
            folds.push(Fold::X(pos));
        } else {
            folds.push(Fold::Y(pos));
        }
    }

    Input { map, folds }
}

fn fold_paper(map: HashSet<(i32, i32)>, fold: &Fold) -> HashSet<(i32, i32)> {
    let mut points = HashSet::<(i32, i32)>::new();

    for (x, y) in map {
        let (x, y) = match fold {
            Fold::X(pos) => (if x >= *pos { pos - (x - pos) } else { x }, y),
            Fold::Y(pos) => (x, if y >= *pos { pos - (y - pos) } else { y }),
        };

        points.insert((x, y));
    }

    points
}

fn task1(input: &Input) -> usize {
    let map = input.map.clone();
    let result = fold_paper(map, &input.folds[0]);

    result.len()
}

fn task2(input: &Input) -> String {
    let mut map = input.map.clone();
    for fold in input.folds.iter() {
        map = fold_paper(map, fold);
    }

    let max_x = map.iter().map(|entry| entry.0).max().unwrap();
    let max_y = map.iter().map(|entry| entry.1).max().unwrap();

    let mut result = vec![];
    for _ in 0..=max_y {
        let mut line = vec![];
        for _ in 0..=max_x {
            line.push(".")
        }
        result.push(line);
    }

    for (x, y) in map.iter() {
        result[*y as usize][*x as usize] = "#";
    }

    let lines: Vec<_> = result.iter().map(|line| line.join("")).collect();
    lines.join("\n")
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: \n{}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
        ",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 17);
    assert_eq!(
        task2(&input),
        "#####
#...#
#...#
#...#
#####"
    );
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 837);
    assert_eq!(
        task2(&input),
        "####.###..####..##..#..#..##..#..#.#..#
#....#..#....#.#..#.#.#..#..#.#..#.#..#
###..#..#...#..#....##...#....####.#..#
#....###...#...#.##.#.#..#....#..#.#..#
#....#....#....#..#.#.#..#..#.#..#.#..#
####.#....####..###.#..#..##..#..#..##."
    )
}
