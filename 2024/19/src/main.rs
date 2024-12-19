use std::{collections::HashMap, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse_input(input: &str) -> Input {
    let mut input = input.trim().lines();
    let patterns = input
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|pattern| pattern.trim().to_string())
        .collect();
    let designs = input.skip(1).map(|line| line.trim().to_string()).collect();

    Input { patterns, designs }
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct HeapElement {
    steps: usize,
    x: i8,
    y: i8,
}

fn can_be_matched(design: &str, patterns: &Vec<String>) -> bool {
    if design.len() == 0 {
        return true;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if can_be_matched(&design[pattern.len()..], patterns) {
                return true;
            }
        }
    }

    return false;
}

fn can_be_maybe_matched(design: &str, patterns: &Vec<Option<&String>>) -> bool {
    if design.len() == 0 {
        return true;
    }

    for maybe_pattern in patterns {
        if let Some(pattern) = maybe_pattern {
            if design.starts_with(*pattern) {
                if can_be_maybe_matched(&design[pattern.len()..], patterns) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn task1(input: &Input) -> usize {
    let mut maybe_patterns: Vec<_> = input.patterns.iter().map(|pattern| Some(pattern)).collect();

    loop {
        let active_patterns = maybe_patterns
            .iter()
            .filter(|pattern| pattern.is_some())
            .count();

        for i in 0..maybe_patterns.len() {
            if let Some(pattern) = maybe_patterns[i] {
                maybe_patterns[i] = None;
                if !can_be_maybe_matched(&pattern, &maybe_patterns) {
                    maybe_patterns[i] = Some(pattern);
                }
            }
        }

        if maybe_patterns.iter().filter_map(|pattern| *pattern).count() == active_patterns {
            break;
        }
    }

    let patterns: Vec<_> = maybe_patterns
        .iter()
        .filter_map(|pattern| *pattern)
        .map(|pattern| pattern.clone())
        .collect();

    input
        .designs
        .iter()
        .filter(|design| can_be_matched(&design[..], &patterns))
        .count()
}

fn count_matches<'a>(
    design: &'a str,
    patterns: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if let Some(count) = cache.get(design) {
        return *count;
    }

    if design.len() == 1 {
        return if patterns.iter().any(|pattern| *pattern == design) {
            cache.insert(design, 1);
            1
        } else {
            cache.insert(design, 0);
            0
        };
    }

    let mid_point = design.len() / 2;

    let mut matches = count_matches(&design[..mid_point], patterns, cache)
        * count_matches(&design[mid_point..], patterns, cache);

    for pattern in patterns {
        for split_position in 1..pattern.len() {
            if mid_point >= split_position {
                let match_start = mid_point - split_position;
                let match_end = match_start + pattern.len();
                if match_end <= design.len() && &design[match_start..match_end] == *pattern {
                    matches += count_matches(&design[..match_start], patterns, cache)
                        * count_matches(&design[match_end..], patterns, cache);
                }
            }
        }
    }

    cache.insert(design, matches);
    matches
}

fn task2(input: &Input) -> usize {
    let mut cache = HashMap::new();

    let patterns = input.patterns.iter().map(|pattern| &pattern[..]).collect();
    input
        .designs
        .iter()
        .map(|design| {
            let matches = count_matches(&design[..], &patterns, &mut cache);
            matches
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
    let test_input1 = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb";

    let input1 = parse_input(test_input1);

    assert_eq!(task1(&input1), 6);
    assert_eq!(task2(&input1), 16);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 369);
    assert_eq!(task2(&input), 761826581538190);
}
