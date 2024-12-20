use std::{collections::HashMap, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Map {
    start: (usize, usize),
    end: (usize, usize),
    walls: Vec<Vec<bool>>,
}

type Input = Map;

fn neighbors(position: (usize, usize)) -> [(usize, usize); 4] {
    [
        (position.0 + 1, position.1),
        (position.0 - 1, position.1),
        (position.0, position.1 + 1),
        (position.0, position.1 - 1),
    ]
}

fn parse_input(input: &str) -> Input {
    let input: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (x, y);
            } else if char == 'E' {
                end = (x, y);
            }
        }
    }

    let walls = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    Map { start, end, walls }
}

fn fill(walls: &Vec<Vec<bool>>, source: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut visited = HashMap::new();
    visited.insert(source, 0);
    let mut todos = vec![source];

    while let Some(todo) = todos.pop() {
        let path_length = visited.get(&todo).unwrap();
        let new_path_length = path_length + 1;

        for new_position in neighbors(todo) {
            if walls[new_position.1][new_position.0] {
                continue;
            }

            if let Some(shortest) = visited.get(&new_position) {
                if *shortest <= new_path_length {
                    continue;
                }
            }

            visited.insert(new_position, new_path_length);
            todos.push(new_position);
        }
    }

    visited
}

fn number_of_cheats(input: &Input, minimum_saving: usize) -> usize {
    let mut valid_cheats = 0;
    let from_start = fill(&input.walls, input.start.clone());
    let from_end = fill(&input.walls, input.end.clone());

    let path_length = from_start.get(&input.end).unwrap();

    for (y, line) in input.walls.iter().enumerate() {
        if y == 0 || y == input.walls.len() - 1 {
            continue;
        }

        for (x, is_wall) in line.iter().enumerate() {
            if x == 0 || x == line.len() - 1 {
                continue;
            }

            if *is_wall {
                for new_position1 in neighbors((x, y)) {
                    for new_position2 in neighbors((x, y)) {
                        if new_position1 == new_position2 {
                            continue;
                        }

                        if input.walls[new_position1.1][new_position1.0]
                            || input.walls[new_position2.1][new_position2.0]
                        {
                            continue;
                        }

                        match (from_start.get(&new_position1), from_end.get(&new_position2)) {
                            (Some(distance1), Some(distance2)) => {
                                let distance = distance1 + distance2 + 2;
                                let saving = *path_length as isize - distance as isize;
                                if saving >= minimum_saving as isize {
                                    valid_cheats += 1;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    valid_cheats
}

fn task1(input: &Input) -> usize {
    number_of_cheats(input, 100)
}

fn number_of_long_cheats(input: &Input, minimum_saving: usize) -> usize {
    let mut valid_cheats = 0;
    let from_start = fill(&input.walls, input.start.clone());
    let from_end = fill(&input.walls, input.end.clone());

    let path_length = from_start.get(&input.end).unwrap();

    let mut diamond = Vec::new();
    for x in -20i32..=20 {
        for y in -20i32..=20 {
            if x.abs() + y.abs() <= 20 && (x != 0 || y != 0) {
                diamond.push((x, y));
            }
        }
    }

    for (y, line) in input.walls.iter().enumerate() {
        if y == 0 || y == input.walls.len() - 1 {
            continue;
        }

        for (x, is_wall) in line.iter().enumerate() {
            if x == 0 || x == line.len() - 1 {
                continue;
            }

            if *is_wall {
                continue;
            }

            for diff in diamond.iter() {
                let target = (x as i32 + diff.0, y as i32 + diff.1);
                if !(1..line.len() as i32).contains(&target.0)
                    || !(1..input.walls.len() as i32).contains(&target.1)
                {
                    continue;
                }

                let target = (target.0 as usize, target.1 as usize);
                if input.walls[target.1][target.0] {
                    continue;
                }

                match (from_start.get(&(x, y)), from_end.get(&target)) {
                    (Some(distance1), Some(distance2)) => {
                        let distance =
                            distance1 + distance2 + diff.0.abs() as usize + diff.1.abs() as usize;
                        let saving = *path_length as isize - distance as isize;
                        if saving >= minimum_saving as isize {
                            valid_cheats += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    valid_cheats
}

fn task2(input: &Input) -> usize {
    number_of_long_cheats(input, 100)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############";

    let input = parse_input(test_input);

    assert_eq!(number_of_cheats(&input, 2), 44);
    assert_eq!(number_of_cheats(&input, 4), 30);
    assert_eq!(number_of_cheats(&input, 6), 16);
    assert_eq!(number_of_cheats(&input, 8), 14);
    assert_eq!(number_of_cheats(&input, 10), 10);
    assert_eq!(number_of_cheats(&input, 12), 8);
    assert_eq!(number_of_cheats(&input, 20), 5);
    assert_eq!(number_of_cheats(&input, 36), 4);
    assert_eq!(number_of_cheats(&input, 38), 3);
    assert_eq!(number_of_cheats(&input, 40), 2);
    assert_eq!(number_of_cheats(&input, 64), 1);

    assert_eq!(number_of_long_cheats(&input, 50), 285);
    assert_eq!(number_of_long_cheats(&input, 52), 253);
    assert_eq!(number_of_long_cheats(&input, 54), 222);
    assert_eq!(number_of_long_cheats(&input, 56), 193);
    assert_eq!(number_of_long_cheats(&input, 58), 154);
    assert_eq!(number_of_long_cheats(&input, 60), 129);
    assert_eq!(number_of_long_cheats(&input, 62), 106);
    assert_eq!(number_of_long_cheats(&input, 64), 86);
    assert_eq!(number_of_long_cheats(&input, 66), 67);
    assert_eq!(number_of_long_cheats(&input, 68), 55);
    assert_eq!(number_of_long_cheats(&input, 70), 41);
    assert_eq!(number_of_long_cheats(&input, 72), 29);
    assert_eq!(number_of_long_cheats(&input, 74), 7);
    assert_eq!(number_of_long_cheats(&input, 76), 3);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1355);
    assert_eq!(task2(&input), 1007335);
}
