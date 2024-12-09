use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Clone)]
struct Input {
    obstacles: Vec<Vec<bool>>,
    height: i16,
    width: i16,
    start_point: (i16, i16),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

fn parse_input(input: &str) -> Input {
    let mut start_point = (0, 0);
    let mut obstacles = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
        let mut row_vector = Vec::new();
        for (x, char) in line.trim().chars().enumerate() {
            let has_obstacle = match char {
                '^' => {
                    start_point = (x as i16, y as i16);
                    false
                }
                '#' => true,
                _ => false,
            };
            row_vector.push(has_obstacle);
        }
        obstacles.push(row_vector);
    }

    let height = obstacles.len() as i16;
    let width = obstacles[0].len() as i16;

    Input {
        obstacles,
        start_point,
        height,
        width,
    }
}

fn find_path(input: &Input) -> HashSet<(i16, i16)> {
    let mut positions = HashSet::new();
    let mut x = input.start_point.0;
    let mut y = input.start_point.1;
    let mut direction = North;

    while (0..input.width).contains(&x) && (0..input.height).contains(&y) {
        positions.insert((x, y));

        let (next_x, next_y) = match direction {
            North => (x, y - 1),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x - 1, y),
        };

        if (0..input.width).contains(&next_x)
            && (0..input.height).contains(&next_y)
            && input.obstacles[next_y as usize][next_x as usize]
        {
            direction = match direction {
                North => East,
                East => South,
                South => West,
                West => North,
            };
        } else {
            (x, y) = (next_x, next_y);
        }
    }

    positions
}

fn task1(input: &Input) -> usize {
    let positions = find_path(input);
    positions.len()
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct State {
    x: i16,
    y: i16,
    direction: Direction,
}

impl State {
    fn step(&mut self, input: &Input) -> Option<(i16, i16)> {
        let (next_x, next_y) = match self.direction {
            North => (self.x, self.y - 1),
            East => (self.x + 1, self.y),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
        };

        if (0..input.width).contains(&next_x)
            && (0..input.height).contains(&next_y)
            && input.obstacles[next_y as usize][next_x as usize]
        {
            self.direction = match self.direction {
                North => East,
                East => South,
                South => West,
                West => North,
            };
            Some((next_x, next_y))
        } else {
            (self.x, self.y) = (next_x, next_y);
            None
        }
    }
}

fn loops_forever(
    input: &Input,
    natural_paths_starts: &mut HashMap<State, (bool, HashMap<(i16, i16), Direction>)>,
    additional_obstacle: (i16, i16),
) -> bool {
    let mut states = HashSet::new();
    let mut state_set_since_additional_obstacle = HashSet::new();
    let mut state_vector_since_additional_obstacle = Vec::new();

    let mut state = State {
        x: input.start_point.0,
        y: input.start_point.1,
        direction: North,
    };

    let mut found_loop = false;
    while (0..input.width).contains(&state.x) && (0..input.height).contains(&state.y) {
        if let Some((cached_has_loop, cached_steps)) = natural_paths_starts.get(&state) {
            if let Some(direction) = cached_steps.get(&additional_obstacle) {
                state = match *direction {
                    North => State {
                        x: additional_obstacle.0,
                        y: additional_obstacle.1 + 1,
                        direction: North,
                    },
                    East => State {
                        x: additional_obstacle.0 - 1,
                        y: additional_obstacle.1,
                        direction: East,
                    },
                    South => State {
                        x: additional_obstacle.0,
                        y: additional_obstacle.1 - 1,
                        direction: South,
                    },
                    West => State {
                        x: additional_obstacle.0 + 1,
                        y: additional_obstacle.1,
                        direction: West,
                    },
                }
            } else {
                return *cached_has_loop;
            }
        }

        if states.contains(&state) {
            found_loop = true;
            break;
        }

        states.insert(state);
        state_set_since_additional_obstacle.insert(state);
        state_vector_since_additional_obstacle.push(state);

        if let Some(obstacle) = state.step(input) {
            if obstacle == additional_obstacle {
                state_vector_since_additional_obstacle.clear();
                state_set_since_additional_obstacle.clear();
            }
        }
    }

    if !found_loop || state_set_since_additional_obstacle.contains(&state) {
        let mut natural_steps = HashMap::new();
        for (counter, state) in state_vector_since_additional_obstacle
            .iter()
            .rev()
            .enumerate()
        {
            natural_steps.insert((state.x, state.y), state.direction);

            if counter % 10 == 0 {
                natural_paths_starts.insert(*state, (found_loop, natural_steps.clone()));
            }
        }
    }

    found_loop
}

fn task2(input: &Input) -> usize {
    let mut result = 0;
    let positions = find_path(input);

    let mut input = input.clone();

    let mut natural_paths_starts = HashMap::new();
    loops_forever(&input, &mut natural_paths_starts, (-1, -1));

    for (x, y) in positions.into_iter() {
        if (x, y) == input.start_point {
            continue;
        }

        if input.obstacles[y as usize][x as usize] {
            continue;
        }

        input.obstacles[y as usize][x as usize] = true;
        if loops_forever(&input, &mut natural_paths_starts, (x, y)) {
            result += 1;
        }
        input.obstacles[y as usize][x as usize] = false;
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
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 41);
    assert_eq!(task2(&input), 6);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 5564);
    assert_eq!(task2(&input), 1976);
}
