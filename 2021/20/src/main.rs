use std::{collections::HashSet, fs, ops::RangeInclusive};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

const NEIGHBORS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone)]
struct Input {
    patterns: Vec<bool>,
    image: Image,
}

#[derive(Debug, Clone)]
struct Image {
    lit_pixels: HashSet<(i32, i32)>,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    infinity_lit: bool,
}

fn parse_input(input: String) -> Input {
    let mut lines = input.trim().lines();
    let mut pattern_string = String::from("");
    loop {
        let line = lines.next().unwrap().trim();
        if line.len() == 0 {
            break;
        }

        pattern_string += line;
    }

    let patterns = pattern_string.chars().map(|char| char == '#').collect();

    let mut lit_pixels = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in lines.enumerate() {
        let line = line.trim();
        height = y as i32;
        width = line.len() as i32 - 1;

        line.chars()
            .enumerate()
            .filter(|(_, char)| *char == '#')
            .for_each(|(x, _)| {
                lit_pixels.insert((x as i32, y as i32));
            });
    }

    let image = Image {
        lit_pixels,
        x_range: 0..=width,
        y_range: 0..=height,
        infinity_lit: false,
    };

    Input { patterns, image }
}

fn step(patterns: &Vec<bool>, image: Image) -> Image {
    let Image {
        lit_pixels,
        x_range,
        y_range,
        infinity_lit,
    } = image;

    let new_x_range = (x_range.start() - 1)..=(x_range.end() + 1);
    let new_y_range = (y_range.start() - 1)..=(y_range.end() + 1);

    let mut new_lit_pixels = HashSet::new();

    for x in new_x_range.clone() {
        for y in new_y_range.clone() {
            let mut index = 0;

            for (diff_x, diff_y) in NEIGHBORS {
                let bit = if !x_range.contains(&(x + diff_x)) || !y_range.contains(&(y + diff_y)) {
                    infinity_lit
                } else {
                    lit_pixels.contains(&(x + diff_x, y + diff_y))
                };
                index = index * 2 + (bit as usize);
            }

            if patterns[index] {
                new_lit_pixels.insert((x, y));
            }
        }
    }

    let new_infinity_lit = patterns[if infinity_lit { 511 } else { 0 }];

    Image {
        lit_pixels: new_lit_pixels,
        x_range: new_x_range,
        y_range: new_y_range,
        infinity_lit: new_infinity_lit,
    }
}

fn task1(input: &Input) -> usize {
    let image = step(&input.patterns, input.image.clone());
    let image = step(&input.patterns, image);

    image.lit_pixels.len()
}

fn task2(input: &Input) -> usize {
    let mut image = input.image.clone();
    for _ in 0..50 {
        image = step(&input.patterns, image);
    }

    image.lit_pixels.len()
}

fn main() {
    let input = parse_input(load_input());
    println!("{}", input.patterns.len());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input(String::from(
        "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
        ",
    ));

    assert_eq!(task1(&input), 35);
    assert_eq!(task2(&input), 3351);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 5483);
    assert_eq!(task2(&input), 18732);
}
