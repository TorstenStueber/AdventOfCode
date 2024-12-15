use std::{collections::HashSet, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

use Direction::*;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn position_diff(&self) -> (i64, i64) {
        match *self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    initial_position: (usize, usize),
    map: Vec<Vec<char>>,
    steps: Vec<Direction>,
}

type Input = Robot;

fn parse_input(input: &str) -> Input {
    let input: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();
    let position = input.iter().position(|line| line.len() == 0).unwrap();

    let (map, steps) = input.split_at(position);
    let steps = steps[1..].to_vec();

    let mut initial_position = (0, 0);

    let mut map: Vec<Vec<_>> = map.iter().map(|line| line.chars().collect()).collect();

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '@' {
                initial_position = (x, y);
            }
        }
    }

    map[initial_position.1 as usize][initial_position.0 as usize] = '.';

    let steps = String::from_iter(steps.into_iter());
    let steps = steps
        .chars()
        .map(|char| match char {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => unreachable!(),
        })
        .collect();

    Robot {
        map,
        steps,
        initial_position,
    }
}

fn task1(input: &Input) -> usize {
    let mut map = input.map.clone();
    let mut position = input.initial_position.clone();

    for step in input.steps.iter() {
        let diff = step.position_diff();
        let new_position = (
            (position.0 as i64 + diff.0) as usize,
            (position.1 as i64 + diff.1) as usize,
        );

        if map[new_position.1][new_position.0] == '#' {
            continue;
        }

        if map[new_position.1][new_position.0] == 'O' {
            let mut box_position = new_position;
            while map[box_position.1][box_position.0] == 'O' {
                box_position.0 = (box_position.0 as i64 + diff.0) as usize;
                box_position.1 = (box_position.1 as i64 + diff.1) as usize;
            }

            if map[box_position.1][box_position.0] == '.' {
                map[box_position.1][box_position.0] = 'O';
                map[new_position.1][new_position.0] = '.'
            }
        }

        if map[new_position.1][new_position.0] == '.' {
            position = new_position;
            continue;
        }
    }

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'O' {
                result += x + 100 * y;
            }
        }
    }

    result
}

fn task2(input: &Input) -> usize {
    let mut map: Vec<Vec<_>> = input
        .map
        .iter()
        .map(|line| {
            let string_parts = line.iter().map(|char| match char {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                _ => unreachable!(),
            });
            String::from_iter(string_parts).chars().collect()
        })
        .collect();

    let mut position = (input.initial_position.0 * 2, input.initial_position.1);

    for step in input.steps.iter() {
        let diff = step.position_diff();
        let new_position = (
            (position.0 as i64 + diff.0) as usize,
            (position.1 as i64 + diff.1) as usize,
        );

        if map[new_position.1][new_position.0] == '#' {
            continue;
        }

        if map[new_position.1][new_position.0] == '.' {
            position = new_position;
            continue;
        }

        if *step == Left || *step == Right {
            let mut box_position = new_position;
            while map[box_position.1][box_position.0] == '['
                || map[box_position.1][box_position.0] == ']'
            {
                box_position.0 = (box_position.0 as i64 + diff.0) as usize;
                box_position.1 = (box_position.1 as i64 + diff.1) as usize;
            }

            if map[box_position.1][box_position.0] == '.' {
                let mut box_position = new_position;
                let mut current_char = '.';

                while map[box_position.1][box_position.0] != '.' {
                    (current_char, map[box_position.1][box_position.0]) =
                        (map[box_position.1][box_position.0], current_char);
                    box_position.0 = (box_position.0 as i64 + diff.0) as usize;
                    box_position.1 = (box_position.1 as i64 + diff.1) as usize;
                }
                map[box_position.1][box_position.0] = current_char;
                position = new_position;
            }
        } else {
            let mut todos: Vec<HashSet<(usize, usize)>> =
                vec![match map[new_position.1][new_position.0] {
                    '[' => HashSet::from_iter([new_position, (new_position.0 + 1, new_position.1)]),
                    ']' => HashSet::from_iter([(new_position.0 - 1, new_position.1), new_position]),
                    _ => unreachable!(),
                }];

            let mut found_collision = false;
            loop {
                let mut next_line = HashSet::new();

                for todo in &todos[todos.len() - 1] {
                    let next = (todo.0, (todo.1 as i64 + diff.1) as usize);

                    match map[next.1][next.0] {
                        '[' => {
                            next_line.insert(next);
                            next_line.insert((next.0 + 1, next.1));
                        }
                        ']' => {
                            next_line.insert((next.0 - 1, next.1));
                            next_line.insert(next);
                        }
                        '#' => {
                            found_collision = true;
                            break;
                        }
                        '.' => (),
                        _ => unreachable!(),
                    }
                }

                if found_collision {
                    break;
                }

                if next_line.is_empty() {
                    break;
                }
                todos.push(next_line);
            }

            if !found_collision {
                position = new_position;

                for todo in todos.iter().rev() {
                    for box_position in todo {
                        let new_box_position =
                            (box_position.0, (box_position.1 as i64 + diff.1) as usize);
                        (
                            map[box_position.1][box_position.0],
                            map[new_box_position.1][new_box_position.0],
                        ) = ('.', map[box_position.1][box_position.0]);
                    }
                }
            }
        }
    }

    let mut result = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == '[' {
                result += x + 100 * y;
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
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<";

    let test_input2 = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);

    assert_eq!(task1(&input1), 2028);
    assert_eq!(task1(&input2), 10092);

    assert_eq!(task2(&input2), 9021);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1465152);
    assert_eq!(task2(&input), 1511259);
}
