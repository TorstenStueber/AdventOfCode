use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

type Input = Vec<Line>;

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("->").unwrap();
            let (x1, y1) = start.trim().split_once(",").unwrap();
            let (x2, y2) = end.trim().split_once(",").unwrap();

            Line {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
            }
        })
        .collect()
}

fn prepare_map(input: &Input) -> (Vec<Vec<u32>>, usize, usize) {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for Line { x1, y1, x2, y2 } in input {
        min_x = usize::min(min_x, usize::min(*x1, *x2));
        min_y = usize::min(min_y, usize::min(*y1, *y2));
        max_x = usize::max(max_x, usize::max(*x1, *x2));
        max_y = usize::max(max_y, usize::max(*y1, *y2));
    }

    let map: Vec<Vec<u32>> = vec![vec![0; max_x - min_x + 1]; max_y - min_y + 1];

    (map, min_x, min_y)
}

fn task1(input: &Input) -> u32 {
    let (mut map, min_x, min_y) = prepare_map(input);

    for line in input {
        if line.x1 == line.x2 {
            for y in usize::min(line.y1, line.y2)..=usize::max(line.y1, line.y2) {
                map[y - min_y][line.x1 - min_x] += 1;
            }
        } else if line.y1 == line.y2 {
            for x in usize::min(line.x1, line.x2)..=usize::max(line.x1, line.x2) {
                map[line.y1 - min_y][x - min_x] += 1;
            }
        }
    }

    let mut count = 0;
    for line in map {
        for field in line {
            if field >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn task2(input: &Input) -> u32 {
    let (mut map, min_x, min_y) = prepare_map(input);

    for line in input {
        if line.x1 == line.x2 {
            for y in usize::min(line.y1, line.y2)..=usize::max(line.y1, line.y2) {
                map[y - min_y][line.x1 - min_x] += 1;
            }
        } else if line.y1 == line.y2 {
            for x in usize::min(line.x1, line.x2)..=usize::max(line.x1, line.x2) {
                map[line.y1 - min_y][x - min_x] += 1;
            }
        } else if line.y2 as isize - line.y1 as isize == line.x2 as isize - line.x1 as isize {
            let line_min_x = usize::min(line.x1, line.x2);
            let line_min_y = usize::min(line.y1, line.y2);
            let line_max_x = usize::max(line.x1, line.x2);

            for step in 0..=line_max_x - line_min_x {
                map[step + line_min_y - min_y][step + line_min_x - min_x] += 1;
            }
        } else if line.y2 as isize - line.y1 as isize == line.x1 as isize - line.x2 as isize {
            let line_min_x = usize::min(line.x1, line.x2);
            let line_min_y = usize::min(line.y1, line.y2);
            let line_max_x = usize::max(line.x1, line.x2);

            for step in 0..=line_max_x - line_min_x {
                map[step + line_min_y - min_y][line_max_x - step - min_x] += 1;
            }
        }
    }

    let mut count = 0;
    for line in map {
        for field in line {
            if field >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 5);
    assert_eq!(task2(&input), 12);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 8111);
    assert_eq!(task2(&input), 22088);
}
