use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

struct Input {
    rules: Vec<(u16, u16)>,
    updates: Vec<Vec<u16>>,
}

fn parse_input(input: &str) -> Input {
    let lines: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();

    let split_position = lines.iter().position(|line| line.is_empty()).unwrap();

    let rules = &lines[..split_position];
    let updates = &lines[split_position + 1..];

    Input {
        rules: rules
            .iter()
            .map(|rule| {
                let parts = rule.split_once("|").unwrap();
                (
                    parts.0.trim().parse().unwrap(),
                    parts.1.trim().parse().unwrap(),
                )
            })
            .collect(),
        updates: updates
            .iter()
            .map(|update| {
                update
                    .split(",")
                    .map(|number| number.trim().parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}

fn is_correctly_ordered(update: &Vec<u16>, rules: &Vec<(u16, u16)>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            if rules
                .iter()
                .any(|rule| rule.0 == update[j] && rule.1 == update[i])
            {
                return false;
            }
        }
    }

    return true;
}

fn task1(input: &Input) -> u16 {
    input
        .updates
        .iter()
        .filter_map(|update| {
            if !is_correctly_ordered(&update, &input.rules) {
                return None;
            }

            Some(update[(update.len() - 1) / 2])
        })
        .sum()
}

fn task2(input: &Input) -> u16 {
    input
        .updates
        .iter()
        .filter_map(|update| {
            if is_correctly_ordered(&update, &input.rules) {
                return None;
            }

            let mut update = update.clone();
            let len = update.len();

            let mut elements_extracted = 0;
            loop {
                for i in 0..update.len() {
                    let is_first = update.iter().all(|element| {
                        *element == update[i]
                            || input
                                .rules
                                .iter()
                                .any(|rule| rule.0 == update[i] && rule.1 == *element)
                    });

                    if is_first {
                        elements_extracted += 1;
                        if elements_extracted == (len + 1) / 2 {
                            return Some(update[i]);
                        }
                        update.remove(i);
                        break;
                    }
                }
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
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 143);
    assert_eq!(task2(&input), 123);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 3608);
    assert_eq!(task2(&input), 4922);
}
