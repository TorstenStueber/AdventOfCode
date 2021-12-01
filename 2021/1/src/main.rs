use std::fs;

fn load_input() -> Vec<u32> {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<u32> = contents
        .trim()
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    return lines;
}

fn count_increments(list: Vec<u32>) -> u32 {
    let mut count = 0;
    let mut previous_entry: Option<u32> = None;
    for line in list {
        if let Some(prev) = previous_entry {
            if prev < line {
                count += 1;
            }
        }
        previous_entry = Some(line);
    }

    count
}

fn task1() {
    let lines = load_input();

    println!("Task 1: {}", count_increments(lines));
}

fn task2() {
    let lines = load_input();
    let windows = lines[..lines.len() - 2]
        .iter()
        .enumerate()
        .map(|(index, current)| current + lines[index + 1] + lines[index + 2])
        .collect();

    println!("Task 2: {}", count_increments(windows));
}

fn main() {
    task1();
    task2();
}
