use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(String, u32, u32, u32)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (lhs, rhs) = line.split_once("seconds, but then must rest for").unwrap();
            let (name, middle) = lhs.split_once("can fly").unwrap();
            let (speed, fly_time) = middle.split_once("km/s for").unwrap();
            let (rest_time, _) = rhs.split_once("seconds").unwrap();
            (
                name.trim().into(),
                speed.trim().parse().unwrap(),
                fly_time.trim().parse().unwrap(),
                rest_time.trim().parse().unwrap(),
            )
        })
        .collect()
}

fn task1(input: &Input, time: u32) -> u32 {
    input
        .iter()
        .fold(0, |max, (_, speed, fly_time, rest_time)| {
            let full_cycles = time / (fly_time + rest_time);
            let remaining_cycle = (time % (fly_time + rest_time)).min(*fly_time);
            max.max(speed * (full_cycles * fly_time + remaining_cycle))
        })
}

fn task2(input: &Input, time: u32) -> u32 {
    let mut positions: Vec<_> = input.iter().map(|_| 0).collect();
    let mut points = positions.clone();

    for second in 0..time {
        let mut leader_position = 0;
        for (index, position) in positions.iter_mut().enumerate() {
            let (_, speed, fly_time, rest_time) = input[index];
            if second % (fly_time + rest_time) < fly_time {
                *position += speed;
            }
            if *position >= leader_position {
                leader_position = *position;
            }
        }
        for (index, position) in positions.iter().enumerate() {
            if *position == leader_position {
                points[index] += 1;
            }
        }
    }
    points.iter().fold(0, |acc, p| acc.max(*p))
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input, 2503));
    println!("Task 2: {}", task2(&input, 2503));
}

#[test]
fn example() {
    let test_input = "
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input, 1000), 1120);
    assert_eq!(task2(&input, 1000), 689);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input, 2503), 2655);
    assert_eq!(task2(&input, 2503), 1059);
}
