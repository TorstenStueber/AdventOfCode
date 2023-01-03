use md5::digest::core_api::CoreWrapper;
use md5::{Digest, Md5, Md5Core};
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = String;

fn parse_input(input: &str) -> Input {
    input.trim().into()
}

fn create_hash<'a>(
    counter: usize,
    cache: &'a mut Vec<[u8; 32]>,
    hasher: &mut CoreWrapper<Md5Core>,
    input: &Input,
    repeats: u32,
) -> &'a [u8; 32] {
    if counter < cache.len() {
        return &cache[counter];
    }

    hasher.update(input);
    hasher.update(counter.to_string());
    let mut hash = hasher.finalize_reset();

    for _ in 0..repeats {
        hash.into_iter().for_each(|c| {
            hasher.update([
                char::from_digit((c >> 4) as u32, 16).unwrap() as u8,
                char::from_digit((c & 0xf) as u32, 16).unwrap() as u8,
            ]);
        });
        hash = hasher.finalize_reset();
    }

    let mut result = [0u8; 32];
    hash.into_iter().enumerate().for_each(|(index, c)| {
        result[index << 1] = char::from_digit((c >> 4) as u32, 16).unwrap() as u8;
        result[(index << 1) + 1] = char::from_digit((c & 0xf) as u32, 16).unwrap() as u8;
    });

    cache.push(result);

    &cache[cache.len() - 1]
}

fn execute(input: &Input, repeats: u32) -> usize {
    let mut counter = 0;
    let mut hasher = Md5::new();
    let mut cache = vec![];

    let mut keys_found = 0;
    loop {
        let hash = create_hash(counter, &mut cache, &mut hasher, input, repeats);
        if let Some(c) = hash
            .windows(3)
            .filter(|window| window[0] == window[1] && window[1] == window[2])
            .map(|window| window[0])
            .next()
        {
            for i in 1..=1000 {
                let hash2 = create_hash(counter + i, &mut cache, &mut hasher, input, repeats);
                if hash2.windows(5).any(|window| {
                    c == window[0]
                        && c == window[1]
                        && c == window[2]
                        && c == window[3]
                        && c == window[4]
                }) {
                    keys_found += 1;
                    if keys_found == 64 {
                        return counter;
                    }
                    break;
                }
            }
        }

        counter += 1;
    }
}

fn task1(input: &Input) -> usize {
    execute(input, 0)
}

fn task2(input: &Input) -> usize {
    execute(input, 2016)
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
        abc
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), 22728);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), 15168);
    assert_eq!(task2(&input), 20864);
}
