use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

type Input = Vec<Ingredient>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (_, rest) = line.split_once(": capacity").unwrap();
            let (capacity, rest) = rest.split_once(", durability").unwrap();
            let (durability, rest) = rest.split_once(", flavor").unwrap();
            let (flavor, rest) = rest.split_once(", texture").unwrap();
            let (texture, calories) = rest.split_once(", calories").unwrap();

            Ingredient {
                capacity: capacity.trim().parse().unwrap(),
                durability: durability.trim().parse().unwrap(),
                flavor: flavor.trim().parse().unwrap(),
                texture: texture.trim().parse().unwrap(),
                calories: calories.trim().parse().unwrap(),
            }
        })
        .collect()
}

fn recurse_1(
    input: &Input,
    current_index: usize,
    current_accumulated: (i32, i32, i32, i32),
    remaining: i32,
    maximum: &mut i32,
) {
    if current_index == input.len() {
        let current_score = current_accumulated.0.max(0)
            * current_accumulated.1.max(0)
            * current_accumulated.2.max(0)
            * current_accumulated.3.max(0);
        *maximum = current_score.max(*maximum);
        return;
    }

    for next_amount in 0..=remaining {
        if current_index == input.len() - 1 && next_amount != remaining {
            continue;
        }

        let next_accumulated = (
            current_accumulated.0 + next_amount * input[current_index].capacity,
            current_accumulated.1 + next_amount * input[current_index].durability,
            current_accumulated.2 + next_amount * input[current_index].flavor,
            current_accumulated.3 + next_amount * input[current_index].texture,
        );
        recurse_1(
            input,
            current_index + 1,
            next_accumulated,
            remaining - next_amount,
            maximum,
        );
    }
}

fn recurse_2(
    input: &Input,
    current_index: usize,
    current_accumulated: (i32, i32, i32, i32, i32),
    remaining: i32,
    maximum: &mut i32,
) {
    if current_index == input.len() {
        if current_accumulated.4 != 500 {
            return;
        }

        let current_score = current_accumulated.0.max(0)
            * current_accumulated.1.max(0)
            * current_accumulated.2.max(0)
            * current_accumulated.3.max(0);
        *maximum = current_score.max(*maximum);
        return;
    }

    for next_amount in 0..=remaining {
        if current_index == input.len() - 1 && next_amount != remaining {
            continue;
        }

        let next_accumulated = (
            current_accumulated.0 + next_amount * input[current_index].capacity,
            current_accumulated.1 + next_amount * input[current_index].durability,
            current_accumulated.2 + next_amount * input[current_index].flavor,
            current_accumulated.3 + next_amount * input[current_index].texture,
            current_accumulated.4 + next_amount * input[current_index].calories,
        );
        recurse_2(
            input,
            current_index + 1,
            next_accumulated,
            remaining - next_amount,
            maximum,
        );
    }
}

fn task1(input: &Input) -> i32 {
    let mut maximum = i32::MIN;

    recurse_1(input, 0, (0, 0, 0, 0), 100, &mut maximum);

    maximum
}

fn task2(input: &Input) -> i32 {
    let mut maximum = i32::MIN;

    recurse_2(input, 0, (0, 0, 0, 0, 0), 100, &mut maximum);

    maximum
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input), 62842880);
    assert_eq!(task2(&input), 57600000);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 222870);
    assert_eq!(task2(&input), 117936);
}
