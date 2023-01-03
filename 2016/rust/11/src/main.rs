use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
enum Element<'a> {
    Microchip(&'a str),
    Generator(&'a str),
}

use Element::*;

type Floor<'a> = Vec<Element<'a>>;

type Input<'a> = Vec<Floor<'a>>;
type State<'a> = (u8, Input<'a>);

fn parse_input(input: &str) -> Input {
    fn parse_element(s: &str) -> Option<Element> {
        let s = s.trim();
        if s.len() == 0 {
            return None;
        }
        if let Some((element, _)) = s.split_once("generator") {
            return Some(Generator(element.split_once("a").unwrap().1.trim().into()));
        }
        if let Some((element, _)) = s.split_once("-compatible microchip") {
            return Some(Microchip(element.split_once("a").unwrap().1.trim().into()));
        }

        panic!()
    }

    let mut iter = input.trim().split("\n").map(|line| {
        let elements = line.split_once("contains").unwrap().1.trim();
        let mut floor = vec![];
        if elements.starts_with("nothing") {
            return floor;
        }

        if let Some((lhs, rhs)) = elements.split_once("and") {
            floor.push(parse_element(rhs).unwrap());
            lhs.split(",")
                .map(parse_element)
                .filter_map(|e| e)
                .for_each(|e| {
                    floor.push(e);
                });
        } else {
            floor.push(parse_element(elements).unwrap());
        }

        floor.sort();
        return floor;
    });

    vec![
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
struct HeapItem<T: Eq + Ord> {
    negative_depth_and_estimate: i32,
    depth: u32,
    item: T,
}

fn a_star_search<T: Ord + Eq + Hash + Clone>(
    initial_state: T,
    is_end_state: fn(&T) -> bool,
    heuristic: fn(&T) -> u32,
    next_states: fn(&T) -> Vec<T>,
) -> Option<u32> {
    let mut seen = HashSet::new();
    seen.insert(initial_state.clone());

    let mut todo = BinaryHeap::<HeapItem<T>>::new();
    todo.push(HeapItem {
        negative_depth_and_estimate: -(heuristic(&initial_state) as i32),
        depth: 0,
        item: initial_state,
    });

    while let Some(HeapItem {
        depth, item: next, ..
    }) = todo.pop()
    {
        let next_states = next_states(&next);
        if next_states.iter().any(is_end_state) {
            return Some(depth + 1);
        }

        for state in next_states {
            if seen.contains(&state) {
                continue;
            }

            seen.insert(state.clone());
            todo.push(HeapItem {
                negative_depth_and_estimate: -((heuristic(&state) + depth + 1) as i32),
                depth: depth + 1,
                item: state,
            });
        }
    }

    None
}

fn is_valid_floor(floor: &Vec<Element>) -> bool {
    if floor.iter().all(|element| match element {
        Microchip(_) => true,
        Generator(_) => false,
    }) {
        return true;
    }

    for chip in floor {
        match chip {
            Microchip(name_chip) => {
                if !floor.iter().any(|e| match e {
                    Microchip(_) => false,
                    Generator(name_generator) => name_chip == name_generator,
                }) {
                    return false;
                }
            }
            Generator(_) => (),
        }
    }

    true
}

fn execute(input: &Input) -> u32 {
    let initial_state: State = (0, input.clone());

    let is_end_state = |(position, floors): &State| -> bool {
        *position == 3 && floors[0].len() == 0 && floors[1].len() == 0 && floors[2].len() == 0
    };

    let heuristic = |(position, floors): &State| -> u32 {
        let position = *position as u32;
        let mut lengths: [u32; 4] = [0; 4];
        lengths
            .iter_mut()
            .enumerate()
            .for_each(|(index, entry)| *entry = floors[index].len() as u32);

        lengths[position as usize] -= 1;
        let min_nonempty_floor = if lengths[0] > 0 {
            0
        } else if lengths[1] > 0 {
            1
        } else if lengths[2] > 0 {
            2
        } else {
            3
        };

        let mut h = lengths[0] * 6 + lengths[1] * 4 + lengths[2] * 2;

        if min_nonempty_floor <= position {
            h -= 3 - position;
        } else {
            h -= 3 - min_nonempty_floor;
            h += min_nonempty_floor - position;
        }
        h
    };

    fn next_states<'a>((position, floors): &State<'a>) -> Vec<State<'a>> {
        let mut new_states = vec![];

        let mut bring_elements = vec![];
        for element in &floors[*position as usize] {
            bring_elements.push(vec![element])
        }

        for i in 0..floors[*position as usize].len() {
            for j in i + 1..floors[*position as usize].len() {
                let element1 = &floors[*position as usize][i];
                let element2 = &floors[*position as usize][j];
                bring_elements.push(vec![element1, element2]);
            }
        }

        for elements in bring_elements {
            let mut old_floor = floors[*position as usize].clone();
            old_floor.retain(|e| e != elements[0] && (elements.len() == 1 || e != elements[1]));
            if !is_valid_floor(&old_floor) {
                continue;
            }

            let new_positions = match position {
                0 => vec![1],
                3 => vec![2],
                1 if floors[0].len() == 0 => vec![position + 1],
                1 => vec![position - 1, position + 1],
                2 if floors[0].len() == 0 && floors[1].len() == 0 => vec![position + 1],
                2 => vec![position - 1, position + 1],
                _ => unreachable!(),
            };

            for new_position in new_positions {
                let mut new_floor = floors[new_position as usize].clone();

                elements.iter().for_each(|e| new_floor.push((*e).clone()));

                if !is_valid_floor(&new_floor) {
                    continue;
                }
                new_floor.sort();

                let mut new_floors: Vec<_> = floors
                    .iter()
                    .enumerate()
                    .map(|(p, f)| {
                        if p == new_position as usize || p == *position as usize {
                            vec![]
                        } else {
                            f.clone()
                        }
                    })
                    .collect();
                new_floors[new_position as usize] = new_floor;
                new_floors[*position as usize] = old_floor.clone();

                let new_state = (new_position, new_floors);

                new_states.push(new_state);
            }
        }

        new_states
    }

    a_star_search(initial_state, is_end_state, heuristic, next_states).unwrap()
}

fn task1(input: &Input) -> u32 {
    execute(input)
}

fn task2(input: &Input) -> u32 {
    let mut input = input.clone();
    input[0].push(Generator("elerium"));
    input[0].push(Microchip("elerium"));
    input[0].push(Generator("dilithium"));
    input[0].push(Microchip("dilithium"));
    execute(&input)
}

fn main() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
        The second floor contains a hydrogen generator.
        The third floor contains a lithium generator.
        The fourth floor contains nothing relevant.
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), 11);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), 31);
    assert_eq!(task2(&input), 55);
}
