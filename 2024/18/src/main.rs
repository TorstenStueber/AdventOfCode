use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(i8, i8)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct HeapElement {
    steps: usize,
    x: i8,
    y: i8,
}

fn find_path(input: &Input, no_of_bytes: usize, size: i8) -> usize {
    let mut map = Vec::with_capacity(size as usize + 1);
    for _ in 0..=size {
        let mut row = Vec::with_capacity(size as usize + 1);
        for _ in 0..=size {
            row.push(false)
        }
        map.push(row);
    }

    for (x, y) in input.iter().take(no_of_bytes) {
        map[*y as usize][*x as usize] = true;
    }

    let mut heap = BinaryHeap::new();

    heap.push(Reverse(HeapElement {
        steps: 0,
        x: 0,
        y: 0,
    }));

    let mut shortest_paths = HashMap::new();
    shortest_paths.insert((0, 0), 0);

    while let Some(Reverse(HeapElement { x, y, steps })) = heap.pop() {
        if (x, y) == (size, size) {
            return steps;
        }

        for diff in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_x = x + diff.0;
            let next_y = y + diff.1;

            if !(0..=size).contains(&next_x)
                || !(0..=size).contains(&next_y)
                || map[next_y as usize][next_x as usize]
            {
                continue;
            }

            let new_steps = steps + 1;
            if let Some(previous_shortest_length) = shortest_paths.get(&(next_x, next_y)) {
                if *previous_shortest_length <= new_steps {
                    continue;
                }
            }

            shortest_paths.insert((next_x, next_y), new_steps);
            heap.push(Reverse(HeapElement {
                steps: new_steps,
                x: next_x,
                y: next_y,
            }));
        }
    }

    panic!();
}

fn task1(input: &Input) -> usize {
    find_path(input, 1024, 70)
}

fn find_blockage(input: &Input, size: i8) -> String {
    let mut map = Vec::with_capacity(size as usize + 1);
    for _ in 0..=size {
        let mut row = Vec::with_capacity(size as usize + 1);
        for _ in 0..=size {
            row.push(false)
        }
        map.push(row);
    }

    for (x, y) in input {
        map[*y as usize][*x as usize] = true;
    }

    let mut todo = Vec::new();
    let mut visited: HashSet<(i8, i8)> = HashSet::new();
    let mut do_later: HashSet<(i8, i8)> = HashSet::from_iter([(0, 0)]);

    for &(x, y) in [(0, 0)].iter().chain(input.iter().rev()) {
        map[y as usize][x as usize] = false;

        if !do_later.contains(&(x, y)) {
            continue;
        }

        do_later.remove(&(x, y));
        visited.insert((x, y));
        todo.push((x, y));

        while let Some((current_x, current_y)) = todo.pop() {
            if (current_x, current_y) == (size, size) {
                return format! {"{},{}", x, y};
            }

            for step in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next = (current_x + step.0, current_y + step.1);

                if visited.contains(&next) {
                    continue;
                }

                if !(0..=size).contains(&next.0) || !(0..=size).contains(&next.1) {
                    continue;
                }

                if map[next.1 as usize][next.0 as usize] {
                    do_later.insert(next);
                    continue;
                }

                visited.insert(next);
                todo.push(next);
            }
        }
    }

    panic!()
}

fn task2(input: &Input) -> String {
    find_blockage(input, 70)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0";

    let input1 = parse_input(test_input1);

    assert_eq!(find_path(&input1, 12, 6), 22);
    assert_eq!(find_blockage(&input1, 6), "6,1");
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 286);
    assert_eq!(task2(&input), "20,64");
}
