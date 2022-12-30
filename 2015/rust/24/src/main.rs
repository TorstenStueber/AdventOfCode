use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<u32>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn has_subset_sums(vec: &[u32], sum1: u32, sum2: u32) -> bool {
    if sum1 == 0 && sum2 == 0 {
        return true;
    }

    if vec.len() == 0 {
        return false;
    }

    let first = vec[0];

    if first <= sum1 && has_subset_sums(&vec[1..], sum1 - first, sum2) {
        return true;
    }
    if first <= sum2 && has_subset_sums(&vec[1..], sum1, sum2 - first) {
        return true;
    }
    has_subset_sums(&vec[1..], sum1, sum2)
}

fn pick_n(
    remaining: &[u32],
    picked: &mut Vec<u32>,
    not_picked: &mut Vec<u32>,
    n: usize,
    sizes: (u32, u32, u32),
    minimum: &mut Option<u128>,
) {
    if remaining.len() == 0 {
        if n > 0 {
            return;
        }
        let sum: u32 = picked.iter().sum();
        if sum != sizes.0 {
            return;
        }

        if has_subset_sums(&not_picked, sizes.1, sizes.2) {
            let entanglement = picked.iter().fold(1, |acc, a| acc * *a as u128);
            *minimum = Some(match minimum {
                Some(m) => entanglement.min(*m),
                None => entanglement,
            })
        }
        return;
    }

    let next = remaining[0];
    if n > 0 {
        picked.push(next);
        pick_n(&remaining[1..], picked, not_picked, n - 1, sizes, minimum);
        picked.pop();
    }

    not_picked.push(next);
    pick_n(&remaining[1..], picked, not_picked, n, sizes, minimum);
    not_picked.pop();
}

fn task1(input: &Input) -> u128 {
    let mut minimum = None;
    let mut picked = vec![];
    let mut non_picked = vec![];

    let sum: u32 = input.iter().sum();
    let third = sum / 3;
    for n in 1..input.len() {
        pick_n(
            &input[..],
            &mut picked,
            &mut non_picked,
            n,
            (third, third, 0),
            &mut minimum,
        );
        if let Some(m) = minimum {
            return m;
        }
    }

    unreachable!()
}

fn task2(input: &Input) -> u128 {
    let mut minimum = None;
    let mut picked = vec![];
    let mut non_picked = vec![];

    let sum: u32 = input.iter().sum();
    let fourth = sum / 4;
    for n in 1..input.len() {
        pick_n(
            &input[..],
            &mut picked,
            &mut non_picked,
            n,
            (fourth, fourth, fourth),
            &mut minimum,
        );
        if let Some(m) = minimum {
            return m;
        }
    }

    unreachable!()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        1
        2
        3
        4
        5
        7
        8
        9
        10
        11
        ";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input), 99);
    assert_eq!(task2(&input), 44);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 10723906903);
    assert_eq!(task2(&input), 74850409);
}
