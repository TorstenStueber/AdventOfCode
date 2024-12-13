use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

type Input = Vec<Machine>;

fn parse_input(input: &str) -> Input {
    let mut machines = Vec::new();
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut prize;

    for line in input.trim().lines() {
        let line = line.trim();
        match line.split_once(':') {
            Some((lhs, rhs)) if lhs == "Button A" => {
                let (x, y) = rhs.trim().split_once(',').unwrap();
                a = (
                    x.trim().split_once('+').unwrap().1.parse().unwrap(),
                    y.trim().split_once('+').unwrap().1.parse().unwrap(),
                );
            }
            Some((lhs, rhs)) if lhs == "Button B" => {
                let (x, y) = rhs.trim().split_once(',').unwrap();
                b = (
                    x.trim().split_once('+').unwrap().1.parse().unwrap(),
                    y.trim().split_once('+').unwrap().1.parse().unwrap(),
                );
            }
            Some((lhs, rhs)) if lhs == "Prize" => {
                let (x, y) = rhs.trim().split_once(',').unwrap();
                prize = (
                    x.trim().split_once('=').unwrap().1.parse().unwrap(),
                    y.trim().split_once('=').unwrap().1.parse().unwrap(),
                );
                machines.push(Machine { a, b, prize })
            }
            _ => (),
        }
    }

    machines
}

fn task1(input: &Input) -> i64 {
    let mut result = 0;

    for &Machine { a, b, prize } in input {
        let denominator = a.0 * b.1 - a.1 * b.0;
        let enumerator_a = prize.0 * b.1 - prize.1 * b.0;
        let enumerator_b = a.0 * prize.1 - a.1 * prize.0;

        if denominator != 0 {
            if enumerator_a % denominator == 0 && enumerator_b % denominator == 0 {
                result += enumerator_a / denominator * 3 + enumerator_b / denominator * 1;
            }
        } else {
            if enumerator_a == 0 {
                let mut min = None;

                for count_a in 0..i64::MAX {
                    if a.0 * count_a > prize.0 {
                        break;
                    }
                    let diff = prize.0 - a.0 * count_a;
                    if diff % b.0 > 0 {
                        continue;
                    }

                    let count_b = diff / b.0;
                    let cost = count_a * 3 + count_b * 1;

                    min = Some(match min {
                        None => cost,
                        Some(old_cost) => i64::min(cost, old_cost),
                    });
                }

                if let Some(min) = min {
                    result += min
                }
            }
        }
    }

    result
}

fn task2(input: &Input) -> i64 {
    let machines = input
        .iter()
        .cloned()
        .map(|Machine { a, b, prize }| Machine {
            a,
            b,
            prize: (prize.0 + 10000000000000, prize.1 + 10000000000000),
        })
        .collect();

    task1(&machines)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 480);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 31065);
    assert_eq!(task2(&input), 93866170395343);
}
