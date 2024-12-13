use std::{collections::HashSet, fs};

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
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut remaining_plots = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            remaining_plots.insert((x, y));
        }
    }

    let mut result = 0;
    while let Some((x, y)) = remaining_plots.iter().next() {
        let region_char = input[*y as usize][*x as usize];
        let mut area = HashSet::new();
        let mut border_y = HashSet::new();
        let mut border_x = HashSet::new();
        let mut todo = vec![(*x, *y)];

        area.insert((*x, *y));

        while let Some((x, y)) = todo.pop() {
            remaining_plots.remove(&(x, y));

            for line in [(x, y), (x + 1, y)] {
                if border_x.contains(&line) {
                    border_x.remove(&line);
                } else {
                    border_x.insert(line);
                }
            }

            for line in [(x, y), (x, y + 1)] {
                if border_y.contains(&line) {
                    border_y.remove(&line);
                } else {
                    border_y.insert(line);
                }
            }

            for (diff_x, diff_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (next_x, next_y) = (x + diff_x, y + diff_y);

                if !(0..width as i32).contains(&next_x) || !(0..height as i32).contains(&next_y) {
                    continue;
                }

                if area.contains(&(next_x, next_y)) {
                    continue;
                }

                if input[next_y as usize][next_x as usize] != region_char {
                    continue;
                }

                area.insert((next_x, next_y));
                todo.push((next_x, next_y));
            }
        }

        result += (border_x.len() + border_y.len()) * area.len();
    }

    result
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Lateral {
    X,
    Y,
}

fn task2(input: &Input) -> usize {
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut remaining_plots = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            remaining_plots.insert((x, y));
        }
    }

    let mut result = 0;
    while let Some((x, y)) = remaining_plots.iter().next() {
        let region_char = input[*y as usize][*x as usize];
        let mut area = HashSet::new();
        let mut border_y = HashSet::new();
        let mut border_x = HashSet::new();
        let mut todo = vec![(*x, *y)];

        area.insert((*x, *y));

        while let Some((x, y)) = todo.pop() {
            remaining_plots.remove(&(x, y));

            for line in [(x, y), (x + 1, y)] {
                if border_x.contains(&line) {
                    border_x.remove(&line);
                } else {
                    border_x.insert(line);
                }
            }

            for line in [(x, y), (x, y + 1)] {
                if border_y.contains(&line) {
                    border_y.remove(&line);
                } else {
                    border_y.insert(line);
                }
            }

            for (diff_x, diff_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (next_x, next_y) = (x + diff_x, y + diff_y);

                if !(0..width as i32).contains(&next_x) || !(0..height as i32).contains(&next_y) {
                    continue;
                }

                if area.contains(&(next_x, next_y)) {
                    continue;
                }

                if input[next_y as usize][next_x as usize] != region_char {
                    continue;
                }

                area.insert((next_x, next_y));
                todo.push((next_x, next_y));
            }
        }

        let mut sides = 0;

        while let Some(&start) = border_x.iter().next() {
            let mut current = start;
            let mut direction = if area.contains(&start) {
                Direction::Up
            } else {
                Direction::Down
            };

            loop {
                let check = match direction {
                    Direction::Up => [
                        (
                            Lateral::Y,
                            (current.0 - 1, current.1),
                            Direction::Left,
                            Some((current.0, current.1 - 1)),
                        ),
                        (
                            Lateral::X,
                            (current.0, current.1 - 1),
                            Direction::Up,
                            Some((current.0, current.1 - 1)),
                        ),
                        (Lateral::Y, (current.0, current.1), Direction::Right, None),
                    ],
                    Direction::Right => [
                        (
                            Lateral::X,
                            (current.0 + 1, current.1 - 1),
                            Direction::Up,
                            Some((current.0 + 1, current.1)),
                        ),
                        (
                            Lateral::Y,
                            (current.0 + 1, current.1),
                            Direction::Right,
                            Some((current.0 + 1, current.1)),
                        ),
                        (
                            Lateral::X,
                            (current.0 + 1, current.1),
                            Direction::Down,
                            None,
                        ),
                    ],
                    Direction::Down => [
                        (
                            Lateral::Y,
                            (current.0, current.1 + 1),
                            Direction::Right,
                            Some((current.0 - 1, current.1 + 1)),
                        ),
                        (
                            Lateral::X,
                            (current.0, current.1 + 1),
                            Direction::Down,
                            Some((current.0 - 1, current.1 + 1)),
                        ),
                        (
                            Lateral::Y,
                            (current.0 - 1, current.1 + 1),
                            Direction::Left,
                            None,
                        ),
                    ],
                    Direction::Left => [
                        (
                            Lateral::X,
                            (current.0, current.1),
                            Direction::Down,
                            Some((current.0 - 1, current.1 - 1)),
                        ),
                        (
                            Lateral::Y,
                            (current.0 - 1, current.1),
                            Direction::Left,
                            Some((current.0 - 1, current.1 - 1)),
                        ),
                        (Lateral::X, (current.0, current.1 - 1), Direction::Up, None),
                    ],
                };

                let mut found_next = false;
                for (lateral, next, next_direction, required_block) in check {
                    let borders = match lateral {
                        Lateral::X => &mut border_x,
                        Lateral::Y => &mut border_y,
                    };

                    if borders.contains(&next)
                        && (required_block.map_or(true, |required| area.contains(&required)))
                    {
                        borders.remove(&next);
                        current = next;
                        if direction != next_direction {
                            sides += 1;
                            direction = next_direction;
                        }
                        found_next = true;
                        break;
                    }
                }

                if !found_next {
                    break;
                }
            }
        }

        result += sides * area.len();
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
        AAAA
        BBCD
        BBCC
        EEEC";

    let test_input2 = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO";

    let test_input3 = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE";

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);
    let input3 = parse_input(test_input3);

    assert_eq!(task1(&input1), 4 * 10 + 4 * 8 + 4 * 10 + 1 * 4 + 3 * 8);
    assert_eq!(task1(&input2), 772);
    assert_eq!(task1(&input3), 1930);

    let test_input4 = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE";

    let test_input5 = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA";

    let input4 = parse_input(test_input4);
    let input5 = parse_input(test_input5);

    assert_eq!(task2(&input1), 80);
    assert_eq!(task2(&input2), 436);
    assert_eq!(task2(&input4), 236);
    assert_eq!(task2(&input5), 368);
    assert_eq!(task2(&input3), 1206);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1483212);
    assert_eq!(task2(&input), 897062);
}
