use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = u32;

fn parse_input(input: &str) -> Input {
    input.trim().parse().unwrap()
}

fn breadth_first_search<T: Ord + Eq + Hash + Clone + Debug, F1, F2>(
    initial_state: T,
    is_end_state: F1,
    next_states: F2,
) -> Option<u32>
where
    F1: Fn(&T) -> bool,
    F2: Fn(&T) -> Vec<T>,
{
    let mut seen = HashSet::new();
    seen.insert(initial_state.clone());

    let mut todo = vec![(initial_state, 0)];
    let mut todo_index = 0;

    while todo_index < todo.len() {
        let (next, depth) = &todo[todo_index];
        let new_depth = depth + 1;

        todo_index += 1;
        let next_states = next_states(&next);
        if next_states.iter().any(|s| is_end_state(s)) {
            return Some(new_depth);
        }

        for state in next_states {
            if seen.contains(&state) {
                continue;
            }

            seen.insert(state.clone());

            todo.push((state, new_depth));
        }
    }

    None
}

fn expand_search<T: Ord + Eq + Hash + Clone + Debug, F>(
    initial_state: T,
    next_states: F,
    max_depth: u32,
) -> usize
where
    F: Fn(&T) -> Vec<T>,
{
    let mut seen = HashSet::new();
    seen.insert(initial_state.clone());

    let mut todo = vec![(initial_state, 0)];
    let mut todo_index = 0;

    while todo_index < todo.len() {
        let (next, depth) = &todo[todo_index];
        let new_depth = depth + 1;
        if new_depth > max_depth {
            break;
        }

        todo_index += 1;
        let next_states = next_states(&next);

        for state in next_states {
            if seen.contains(&state) {
                continue;
            }

            seen.insert(state.clone());

            todo.push((state, new_depth));
        }
    }

    seen.len()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct State(u32, u32);

fn task1(input: &Input, x: u32, y: u32) -> u32 {
    let initial_state = State(1, 1);

    let is_end_state = |position: &State| -> bool { *position == State(x, y) };

    let next_states = |State(x, y): &State| -> Vec<State> {
        let mut next_states = vec![State(x + 1, *y), State(*x, y + 1)];
        if *x > 0 {
            next_states.push(State(x - 1, *y));
        }
        if *y > 0 {
            next_states.push(State(*x, y - 1));
        }

        next_states
            .into_iter()
            .filter(|State(x, y)| {
                let mut value = x * x + 3 * x + 2 * x * y + y + y * y + input;
                let mut bits = 0;
                while value > 0 {
                    if value & 1 == 1 {
                        bits += 1
                    };
                    value >>= 1;
                }
                bits & 1 == 0
            })
            .collect()
    };

    breadth_first_search(initial_state, is_end_state, next_states).unwrap()
}

fn task2(input: &Input) -> usize {
    let initial_state = State(1, 1);

    let next_states = |State(x, y): &State| -> Vec<State> {
        let mut next_states = vec![State(x + 1, *y), State(*x, y + 1)];
        if *x > 0 {
            next_states.push(State(x - 1, *y));
        }
        if *y > 0 {
            next_states.push(State(*x, y - 1));
        }

        next_states
            .into_iter()
            .filter(|State(x, y)| {
                let mut value = x * x + 3 * x + 2 * x * y + y + y * y + input;
                let mut bits = 0;
                while value > 0 {
                    if value & 1 == 1 {
                        bits += 1
                    };
                    value >>= 1;
                }
                bits & 1 == 0
            })
            .collect()
    };

    expand_search(initial_state, next_states, 50)
}

fn main() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    println!("Task 1: {}", task1(&input, 31, 39));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        10
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input, 7, 4), 11);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input, 31, 39), 86);
    assert_eq!(task2(&input), 127);
}
