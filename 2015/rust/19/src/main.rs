use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = (HashMap<Vec<char>, u8>, Vec<u8>, Vec<(u8, Vec<u8>)>);

fn parse_input(input: &str) -> Input {
    let iter = input.trim().split("\n");
    let count = iter.clone().count();
    let mut map = HashMap::new();

    fn lookup(map: &mut HashMap<Vec<char>, u8>, chemical: &[char]) -> u8 {
        let chemical = chemical.to_vec();
        if let Some(code) = map.get(&chemical) {
            return *code;
        }
        let len = map.len() as u8;
        map.insert(chemical, len);
        len
    }

    fn replace_chemicals(map: &mut HashMap<Vec<char>, u8>, chemicals: &str) -> Vec<u8> {
        let chemicals: Vec<_> = chemicals.chars().collect();
        let mut result = vec![];

        let mut start_index = 0;
        for (index, c) in chemicals.iter().enumerate() {
            if c.is_uppercase() && start_index != index {
                result.push(lookup(map, &chemicals[start_index..index]));
                start_index = index;
            }
        }

        if start_index != chemicals.len() {
            result.push(lookup(map, &chemicals[start_index..]))
        }

        result
    }

    let rules = iter
        .clone()
        .take(count - 2)
        .map(|line| {
            let (a, b) = line.split_once("=>").unwrap();
            (
                replace_chemicals(&mut map, a.trim())[0],
                replace_chemicals(&mut map, b.trim()),
            )
        })
        .collect();

    let source = replace_chemicals(&mut map, iter.skip(count - 1).next().unwrap().trim());

    (map, source, rules)
}

fn task1(input: &Input) -> usize {
    let (_, target, rules) = input;
    let mut results = HashSet::new();

    for rule in rules.iter() {
        for (index, part) in target.iter().enumerate() {
            if *part == rule.0 {
                let result: Vec<_> = target
                    .iter()
                    .take(index)
                    .chain(rule.1.iter())
                    .chain(target.iter().skip(index + 1))
                    .collect();
                results.insert(result);
            }
        }
    }

    results.len()
}

fn recurse_match(
    best_span_creations: &Vec<Vec<Vec<Option<u16>>>>,
    rhs: &Vec<u8>,
    rhs_index: usize,
    current_span: usize,
    span_end: usize,
    current_used_steps: u16,
    best: &mut Option<u16>,
) {
    if rhs_index == rhs.len() {
        if current_span == span_end {
            if let Some(current_best) = *best {
                *best = Some(current_best.max(current_used_steps));
            } else {
                *best = Some(current_used_steps);
            }
        }

        return;
    }

    let current_chemical = rhs[rhs_index];
    if rhs_index == rhs.len() - 1 {
        if let Some(steps) = best_span_creations[current_span][span_end][current_chemical as usize]
        {
            recurse_match(
                best_span_creations,
                rhs,
                rhs_index + 1,
                span_end,
                span_end,
                current_used_steps + steps,
                best,
            );
        }

        return;
    }

    for next_span in current_span + 1..span_end {
        if let Some(steps) = best_span_creations[current_span][next_span][current_chemical as usize]
        {
            recurse_match(
                best_span_creations,
                rhs,
                rhs_index + 1,
                next_span,
                span_end,
                current_used_steps + steps,
                best,
            );
        }
    }
}

fn task2(input: &Input) -> u16 {
    let (chemicals_map, target, rules) = input;

    let mut best_span_creations: Vec<_> = (0..target.len())
        .map(|_| {
            let end_span: Vec<_> = (0..=target.len())
                .map(|_| {
                    let best: Vec<Option<u16>> = (0..chemicals_map.len()).map(|_| None).collect();
                    best
                })
                .collect();
            end_span
        })
        .collect();

    for (span_start, chemical) in target.iter().enumerate() {
        best_span_creations[span_start][span_start + 1][*chemical as usize] = Some(0);
    }

    for span in 2..=target.len() {
        for span_start in 0..=target.len() - span {
            let span_end = span_start + span;

            for (lhs, rhs) in rules {
                let mut best = None;

                recurse_match(
                    &best_span_creations,
                    rhs,
                    0,
                    span_start,
                    span_end,
                    1,
                    &mut best,
                );

                if let Some(best) = best {
                    let new_best = if let Some(current_best) =
                        best_span_creations[span_start][span_end][*lhs as usize]
                    {
                        current_best.min(best)
                    } else {
                        best
                    };
                    best_span_creations[span_start][span_end][*lhs as usize] = Some(new_best);
                }
            }
        }
    }

    best_span_creations[0][target.len()][*chemicals_map.get(&vec!['e']).unwrap() as usize].unwrap()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        H => HO
        H => OH
        O => HH
        
        HOH";

    let test_input2 = "
        H => HO
        H => OH
        O => HH
        
        HOHOHO";

    let input1 = parse_input(&test_input1);
    let input2 = parse_input(&test_input2);
    assert_eq!(task1(&input1), 4);
    assert_eq!(task1(&input2), 7);

    let test_input1 = "
        H => HO
        H => OH
        O => HH
        e => H
        e => O
        
        HOH";

    let test_input2 = "
        H => HO
        H => OH
        O => HH
        e => H
        e => O
        
        HOHOHO";

    let input1 = parse_input(&test_input1);
    let input2 = parse_input(&test_input2);
    assert_eq!(task2(&input1), 3);
    assert_eq!(task2(&input2), 6);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 535);
    assert_eq!(task2(&input), 212);
}
