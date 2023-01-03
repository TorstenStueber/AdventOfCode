use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(i64, i64)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (lhs, rhs) = line
                .split_once("positions; at time=0, it is at position")
                .unwrap();
            let lhs = lhs.split_once("has").unwrap().1;
            let rhs = rhs.split_once(".").unwrap().0;
            (lhs.trim().parse().unwrap(), rhs.trim().parse().unwrap())
        })
        .collect()
}

fn extended_euclidian_algorithm(a: i64, b: i64) -> (i64, i64) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let next_r = r.0 % r.1;
        let q = (r.0 - next_r) / r.1;

        s = (s.1, s.0 - s.1 * q);
        t = (t.1, t.0 - t.1 * q);
        r = (r.1, r.0 - r.1 * q);
    }

    (s.0, t.0)
}

fn chinese_remainder_theorem(inputs: Vec<(i64, i64)>) -> i64 {
    let (mut n, mut k) = inputs[0];

    for (n_current, k_current) in inputs.iter().skip(1) {
        let (s, t) = extended_euclidian_algorithm(n, *n_current);
        k = k_current * s * n + k * t * n_current;
        n = n * n_current;
        k = ((k % n) + n) % n;
    }

    k
}

fn task1(input: &Input) -> i64 {
    chinese_remainder_theorem(
        input
            .iter()
            .enumerate()
            .map(|(index, (frequency, offset))| (*frequency, -(offset + index as i64 + 1)))
            .collect(),
    )
}

fn task2(input: &Input) -> i64 {
    let mut input = input.clone();
    input.push((11, 0));
    task1(&input)
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
        Disc #1 has 5 positions; at time=0, it is at position 4.
        Disc #2 has 2 positions; at time=0, it is at position 1.
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), 5);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), 400589);
    assert_eq!(task2(&input), 3045959);
}
