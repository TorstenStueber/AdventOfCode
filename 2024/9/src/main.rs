use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<usize>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect()
}

fn task1(input: &Input) -> usize {
    let mut result = 0;
    let mut start_index = 0;
    let mut end_index = (input.len() - 1) / 2;

    let mut start_counter;
    let mut end_counter = 0;

    let mut index = 0;
    'outer: loop {
        let block_size = input[start_index * 2];
        start_counter = 0;
        while start_counter < block_size {
            if start_index == end_index && start_counter + end_counter >= block_size {
                break 'outer;
            }
            result += index * start_index;
            start_counter += 1;
            index += 1;
        }

        if start_index * 2 + 1 >= input.len() {
            break;
        }

        let mut gap_size = input[start_index * 2 + 1];
        while gap_size > 0 {
            if end_counter >= input[end_index * 2] {
                end_index -= 1;
                if end_index < start_index {
                    break;
                }
                end_counter = 0;
            }

            result += index * end_index;
            gap_size -= 1;
            index += 1;
            end_counter += 1;
        }

        start_index += 1;

        if end_index < start_index {
            break;
        }
    }

    result
}

#[derive(Debug)]
struct Block {
    files: Vec<(usize, usize)>,
    gap: usize,
    moved: bool,
}

fn task2(input: &Input) -> usize {
    let mut blocks = Vec::new();

    for i in 0..(input.len() + 1) / 2 {
        blocks.push(Block {
            files: vec![(input[i * 2], i)],
            gap: if i * 2 + 1 >= input.len() {
                0
            } else {
                input[i * 2 + 1]
            },
            moved: false,
        })
    }

    for j in (0..blocks.len()).rev() {
        for i in 0..j {
            if blocks[i].gap >= blocks[j].files[0].0 {
                blocks[j].moved = true;
                let file = blocks[j].files[0];
                blocks[i].files.push(file);
                blocks[i].gap -= blocks[j].files[0].0;
                break;
            }
        }
    }

    let mut result = 0;
    let mut index = 0;
    for block in blocks {
        if block.moved {
            index += block.files[0].0;
        }

        for file in block.files.iter().skip(if block.moved { 1 } else { 0 }) {
            for _ in 0..file.0 {
                result += index * file.1;
                index += 1;
            }
        }
        index += block.gap;
    }

    result
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        2333133121414131402";

    let input = parse_input(test_input);

    assert_eq!(task1(&input), 1928);
    assert_eq!(task2(&input), 2858);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 6435922584968);
    assert_eq!(task2(&input), 6469636832766);
}
