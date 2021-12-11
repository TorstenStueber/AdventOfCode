use std::convert::TryInto;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Entry {
    patterns: [String; 10],
    output: [String; 4],
}

type Input = Vec<Entry>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(patterns, output)| {
            let patterns: Vec<_> = patterns.split_whitespace().map(String::from).collect();
            let output: Vec<_> = output.split_whitespace().map(String::from).collect();

            Entry {
                patterns: patterns.try_into().unwrap(),
                output: output.try_into().unwrap(),
            }
        })
        .collect()
}

fn task1(input: &Input) -> i32 {
    input
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|pattern| pattern.len() != 5 && pattern.len() != 6)
                .count() as i32
        })
        .sum()
}

fn intersect(left: &String, right: &str) -> usize {
    left.chars().filter(|&char| right.contains(char)).count()
}

fn matches(left: &String, right: &str) -> bool {
    left.len() == right.len() && intersect(left, right) == left.len()
}

fn task2(input: &Input) -> u32 {
    input
        .iter()
        .map(|entry| {
            let mut digits = [""; 10];

            for pattern in entry.patterns.iter() {
                match pattern.len() {
                    2 => digits[1] = pattern,
                    3 => digits[7] = pattern,
                    4 => digits[4] = pattern,
                    7 => digits[8] = pattern,
                    _ => (),
                }
            }

            for pattern in entry.patterns.iter() {
                if pattern.len() == 6 && intersect(pattern, digits[4]) == 4 {
                    digits[9] = pattern;
                }
                if pattern.len() == 5 && intersect(pattern, digits[1]) == 2 {
                    digits[3] = pattern;
                }
            }

            for pattern in entry.patterns.iter() {
                if pattern.len() == 6 && intersect(pattern, digits[7]) == 2 {
                    digits[6] = pattern;
                }
                if pattern.len() == 5 && intersect(pattern, digits[4]) == 2 {
                    digits[2] = pattern;
                }
            }

            for pattern in entry.patterns.iter() {
                if pattern.len() == 6 && pattern != digits[6] && pattern != digits[9] {
                    digits[0] = pattern;
                }
                if pattern.len() == 5 && pattern != digits[3] && pattern != digits[2] {
                    digits[5] = pattern;
                }
            }

            let mut n = 0;
            for output in entry.output.iter() {
                n = n * 10
                    + digits
                        .iter()
                        .enumerate()
                        .find(|(_, &digit)| matches(output, digit))
                        .unwrap()
                        .0 as u32;
            }

            n
        })
        .sum()
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 26);
    assert_eq!(task2(&input), 61229);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 532);
    assert_eq!(task2(&input), 1011284);
}
