use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

use Direction::*;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Copy)]
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

    fn rotate_clockwise(&self) -> Direction {
        match *self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    start: (i64, i64),
    end: (i64, i64),
    walls: Vec<Vec<bool>>,
}

type Input = Map;

fn parse_input(input: &str) -> Input {
    let input: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (x as i64, y as i64);
            } else if char == 'E' {
                end = (x as i64, y as i64);
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

fn task1(input: &Input) -> i64 {
    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
    struct HeapElement {
        weight: i64,
        position: (i64, i64),
        direction: Direction,
        weight_upto: i64,
    }

    let mut heap = BinaryHeap::new();

    heap.push(Reverse(HeapElement {
        weight: (input.start.0 - input.end.0).abs() + (input.start.1 - input.end.1).abs(),
        position: input.start,
        direction: Right,
        weight_upto: 0,
    }));

    let mut done = HashMap::new();
    done.insert((Right, input.start.0, input.start.1), 0);

    while let Some(element) = heap.pop() {
        if element.0.position == input.end {
            return element.0.weight;
        }

        let direction_diff = element.0.direction.position_diff();
        let next_state1 = (
            element.0.direction,
            element.0.position.0 + direction_diff.0,
            element.0.position.1 + direction_diff.1,
            1,
        );

        let next_state2 = (
            element.0.direction.rotate_clockwise(),
            element.0.position.0,
            element.0.position.1,
            1000,
        );

        let next_state3 = (
            element.0.direction.rotate_counter_clockwise(),
            element.0.position.0,
            element.0.position.1,
            1000,
        );

        for state in [next_state1, next_state2, next_state3] {
            if let Some(value) = done.get(&(state.0, state.1, state.2)) {
                if *value <= element.0.weight_upto + state.3 {
                    continue;
                }
            }

            if input.walls[state.2 as usize][state.1 as usize] {
                continue;
            }

            done.insert((state.0, state.1, state.2), element.0.weight_upto + state.3);

            heap.push(Reverse(HeapElement {
                weight: element.0.weight_upto
                    + state.3
                    + (state.1 - input.end.0).abs()
                    + (state.2 - input.end.1).abs(),
                position: (state.1, state.2),
                direction: state.0,
                weight_upto: element.0.weight_upto + state.3,
            }));
        }
    }

    unreachable!()
}

fn task2(input: &Input) -> usize {
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct HeapElement {
        weight: i64,
        position: (i64, i64),
        direction: Direction,
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse(HeapElement {
        weight: 0,
        position: input.start,
        direction: Right,
    }));

    let mut best_paths = HashMap::new();
    best_paths.insert((input.start, Right), 0);

    let mut maybe_lowest_weight = None;

    while let Some(smallest_element) = heap.pop() {
        if let Some(lowest_weight) = maybe_lowest_weight {
            if smallest_element.0.weight > lowest_weight {
                break;
            }
        }

        if smallest_element.0.position == input.end {
            maybe_lowest_weight = Some(smallest_element.0.weight);
            continue;
        }

        let step_diff = smallest_element.0.direction.position_diff();
        let next_state1 = HeapElement {
            weight: smallest_element.0.weight + 1,
            position: (
                smallest_element.0.position.0 + step_diff.0,
                smallest_element.0.position.1 + step_diff.1,
            ),
            direction: smallest_element.0.direction,
        };

        let next_state2 = HeapElement {
            weight: smallest_element.0.weight + 1000,
            position: smallest_element.0.position,
            direction: smallest_element.0.direction.rotate_clockwise(),
        };

        let next_state3 = HeapElement {
            weight: smallest_element.0.weight + 1000,
            position: smallest_element.0.position,
            direction: smallest_element.0.direction.rotate_counter_clockwise(),
        };

        for next_state in [next_state1, next_state2, next_state3] {
            if input.walls[next_state.position.1 as usize][next_state.position.0 as usize] {
                continue;
            }

            if let Some(previous_weight) =
                best_paths.get(&(next_state.position, next_state.direction))
            {
                if *previous_weight <= next_state.weight {
                    continue;
                }
            }

            best_paths.insert(
                (next_state.position, next_state.direction),
                next_state.weight,
            );

            heap.push(Reverse(next_state));
        }
    }

    let mut done = HashSet::new();
    let mut todos = Vec::new();

    for direction in [Up, Right, Down, Left] {
        if let Some(weight) = best_paths.get(&(input.end, direction)) {
            if Some(*weight) == maybe_lowest_weight {
                todos.push((input.end, direction, *weight));
                done.insert((input.end, direction));
            }
        }
    }

    while let Some((position, direction, weight)) = todos.pop() {
        let step_diff = direction.position_diff();
        let previous_state1 = HeapElement {
            weight: weight - 1,
            position: (position.0 - step_diff.0, position.1 - step_diff.1),
            direction: direction,
        };

        let previous_state2 = HeapElement {
            weight: weight - 1000,
            position: position,
            direction: direction.rotate_clockwise(),
        };

        let previous_state3 = HeapElement {
            weight: weight - 1000,
            position: position,
            direction: direction.rotate_counter_clockwise(),
        };

        for HeapElement {
            weight: expected_weight,
            position,
            direction,
        } in [previous_state1, previous_state2, previous_state3]
        {
            if let Some(weight) = best_paths.get(&(position, direction)) {
                if *weight == expected_weight {
                    if done.contains(&(position, direction)) {
                        continue;
                    }

                    todos.push((position, direction, expected_weight));
                    done.insert((position, direction));
                }
            }
        }
    }

    let best_positions: HashSet<_> = done.iter().map(|(position, _)| position).collect();
    best_positions.len()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############";

    let test_input2 = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################";

    let input1 = parse_input(test_input1);
    let input2 = parse_input(test_input2);

    assert_eq!(task1(&input1), 7036);
    assert_eq!(task1(&input2), 11048);

    assert_eq!(task2(&input1), 45);
    assert_eq!(task2(&input2), 64);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 72428);
    assert_eq!(task2(&input), 456);
}
