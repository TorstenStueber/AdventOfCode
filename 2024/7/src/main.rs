use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(u64, Vec<u64>)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (total, terms) = line.trim().split_once(":").unwrap();
            (
                total.parse().unwrap(),
                terms
                    .trim()
                    .split_ascii_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn task1(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|(total, terms)| {
            assert!(terms.len() - 1 < 32);
            assert!(terms.len() > 1);
            for combination in 0..(1 << (terms.len() - 1)) {
                let computed_total =
                    terms
                        .iter()
                        .skip(1)
                        .enumerate()
                        .fold(terms[0], |acc, (i, term)| {
                            if (combination >> i) & 1 == 1 {
                                acc + term
                            } else {
                                acc * term
                            }
                        });

                if computed_total == *total {
                    return Some(total);
                }
            }
            None
        })
        .sum()
}

fn task2(input: &Input) -> u64 {
    fn can_match(target: u64, values: &[u64]) -> bool {
        assert!(values.len() > 0);
        let last = values[values.len() - 1];
        if last > target {
            return false;
        }
        if values.len() == 1 {
            return last == target;
        }

        if format!("{}", target).ends_with(&format!("{}", last)) {
            let matches = can_match(
                target / (10u64.pow(format!("{}", last).len() as u32)),
                &values[..values.len() - 1],
            );

            if matches {
                return true;
            }
        }

        if target % last == 0 {
            let matches = can_match(target / last, &values[..values.len() - 1]);

            if matches {
                return true;
            }
        }

        return can_match(target - last, &values[..values.len() - 1]);
    }

    input
        .iter()
        .filter_map(|(total, terms)| {
            if can_match(*total, terms) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 3749);
    assert_eq!(task2(&input), 11387);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 3351424677624);
    assert_eq!(task2(&input), 204976636995111);
}
