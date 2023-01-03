use md5::{Digest, Md5};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    input.trim().into()
}

fn breadth_first_search<T: Ord + Eq + Hash + Clone + Debug, F1, F2>(
    initial_state: T,
    is_end_state: F1,
    next_states: F2,
) -> Option<(T, u32)>
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
        for s in &next_states {
            if is_end_state(s) {
                return Some((s.clone(), new_depth));
            }
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

fn depth_first_search<T: Ord + Eq + Hash + Clone + Debug, F1, F2>(
    initial_state: T,
    is_end_state: F1,
    next_states: F2,
    max_depth: &mut u32,
) where
    F1: Fn(&T) -> bool,
    F2: Fn(&T) -> Vec<T>,
{
    let mut seen = HashSet::new();
    seen.insert(initial_state.clone());

    let mut todo = vec![(initial_state, 0)];

    while let Some((next, depth)) = todo.pop() {
        let new_depth = depth + 1;

        let next_states = next_states(&next);

        for state in next_states {
            if is_end_state(&state) {
                *max_depth = (*max_depth).max(new_depth);
                continue;
            }

            if seen.contains(&state) {
                continue;
            }

            seen.insert(state.clone());

            todo.push((state, new_depth));
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

use Direction::*;

impl Direction {
    fn to_char(&self) -> u8 {
        match self {
            U => 'U' as u8,
            D => 'D' as u8,
            L => 'L' as u8,
            R => 'R' as u8,
        }
    }

    fn diff(&self) -> (i32, i32) {
        match self {
            U => (0, -1),
            D => (0, 1),
            L => (-1, 0),
            R => (1, 0),
        }
    }

    fn door_is_open(&self, hash: &[u8; 2]) -> bool {
        match self {
            U => hash[0] >> 4 >= 0xb,
            D => hash[0] & 0xf >= 0xb,
            L => hash[1] >> 4 >= 0xb,
            R => hash[1] & 0xf >= 0xb,
        }
    }
}

type State = (i32, i32, Vec<Direction>);

fn next_states((x, y, path): &State, input: &Input) -> Vec<State> {
    let mut hasher = Md5::new();
    hasher.update(input);
    hasher.update(path.iter().map(|d| d.to_char()).collect::<Vec<_>>());
    let hash = hasher.finalize();
    let hash = [hash[0], hash[1]];

    let mut next_states = vec![];

    [U, D, L, R]
        .iter()
        .filter_map(|direction| {
            if !direction.door_is_open(&hash) {
                return None;
            }
            let (diff_x, diff_y) = direction.diff();
            let (new_x, new_y) = (x + diff_x, y + diff_y);
            if new_x >= 0 && new_x <= 3 && new_y >= 0 && new_y <= 3 {
                Some((new_x, new_y, direction))
            } else {
                None
            }
        })
        .for_each(|(new_x, new_y, direction)| {
            let mut new_path = path.clone();
            new_path.push(direction.clone());
            next_states.push((new_x, new_y, new_path));
        });

    next_states
}

fn is_end_state((x, y, _): &State) -> bool {
    *x == 3 && *y == 3
}

fn task1(input: &Input) -> String {
    let initial_state = (0, 0, vec![]);

    let next_states_for_input = |state: &State| -> Vec<State> { next_states(state, input) };

    let ((_, _, path), _) =
        breadth_first_search(initial_state, is_end_state, next_states_for_input).unwrap();

    path.iter().map(|d| d.to_char() as char).collect()
}

fn task2(input: &Input) -> u32 {
    let initial_state = (0, 0, vec![]);

    let next_states_for_input = |state: &State| -> Vec<State> { next_states(state, input) };

    let mut maximum_depth = u32::MIN;
    depth_first_search(
        initial_state,
        is_end_state,
        next_states_for_input,
        &mut maximum_depth,
    );

    maximum_depth
}

fn main() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input("ihgpwlah");
    assert_eq!(task1(&input), "DDRRRD");
    assert_eq!(task2(&input), 370);

    let input = parse_input("kglvqrro");
    assert_eq!(task1(&input), "DDUDRLRRUDRD");
    assert_eq!(task2(&input), 492);

    let input = parse_input("ulqzkmiv");
    assert_eq!(task1(&input), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    assert_eq!(task2(&input), 830);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), "DRRDRLDURD");
    assert_eq!(task2(&input), 618);
}
