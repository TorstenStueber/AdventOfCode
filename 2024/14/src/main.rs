use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    speed: (i64, i64),
}

type Input = Vec<Robot>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let (position, speed) = line.split_once(' ').unwrap();
            let position = position.trim().split_once('=').unwrap().1;
            let position = position.split_once(',').unwrap();
            let speed = speed.trim().split_once('=').unwrap().1;
            let speed = speed.trim().split_once(',').unwrap();

            Robot {
                position: (position.0.parse().unwrap(), position.1.parse().unwrap()),
                speed: (speed.0.parse().unwrap(), speed.1.parse().unwrap()),
            }
        })
        .collect()
}

fn simulate(input: &Input, width: i64, height: i64, steps: i64) -> i64 {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;

    for robot in input {
        let x = (robot.position.0 + robot.speed.0 * steps) % width;
        let y = (robot.position.1 + robot.speed.1 * steps) % height;

        let x = (x + width) % width;
        let y = (y + height) % height;

        if x < width / 2 {
            if y < height / 2 {
                count1 += 1;
            } else if y > height / 2 {
                count2 += 1;
            }
        } else if x > width / 2 {
            if y < height / 2 {
                count3 += 1;
            } else if y > height / 2 {
                count4 += 1;
            }
        }
    }

    count1 * count2 * count3 * count4
}

fn task1(input: &Input) -> i64 {
    simulate(input, 101, 103, 100)
}

fn task2(input: &Input) -> i64 {
    let width = 101;
    let height = 103;
    let mut steps = 0;

    loop {
        let mut lines = [['.'; 101]; 103];
        let mut has_duplicate = false;

        for robot in input {
            let x = (robot.position.0 + robot.speed.0 * steps) % width;
            let y = (robot.position.1 + robot.speed.1 * steps) % height;

            let x = (x + width) % width;
            let y = (y + height) % height;

            if lines[y as usize][x as usize] != '.' {
                has_duplicate = true;
            } else {
                lines[y as usize][x as usize] = '#';
            }
        }

        if !has_duplicate {
            for line in lines {
                println!("{}", String::from_iter(line));
            }

            return steps;
        }
        steps += 1;
    }
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3";

    let input = parse_input(test_input);

    assert_eq!(simulate(&input, 11, 7, 100), 12);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 230436441);
    assert_eq!(task2(&input), 8270);
}
