use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
enum Number {
    Leaf(u32),
    Composite(Box<(Number, Number)>),
}

type Input = Vec<Number>;

impl Number {
    fn from_string(string: &str) -> (Number, &str) {
        let string = string.trim_start();

        if string.starts_with("[") {
            let (left, string) = Number::from_string(&string[1..]);
            let string = string.trim_start();
            if !string.starts_with(",") {
                panic!("Number string malformed");
            }
            let (right, string) = Number::from_string(&string[1..]);
            let string = string.trim_start();
            if !string.starts_with("]") {
                panic!("Number string malformed");
            }
            return (Number::Composite(Box::new((left, right))), &string[1..]);
        }

        let mut chars = string.chars().enumerate();
        let mut number = 0;
        let position = loop {
            if let Some((position, char)) = chars.next() {
                if let Some(digit) = char.to_digit(10) {
                    number = 10 * number + digit;
                } else {
                    break position;
                }
            } else {
                break string.len();
            }
        };

        return (Number::Leaf(number), &string[position..]);
    }

    fn explode(&mut self, depth: u8) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Number::Leaf(_) => None,
            Number::Composite(b) => {
                let (left, right) = (&mut b.0, &mut b.1);
                if depth == 4 {
                    let left = left.as_number();
                    let right = right.as_number();

                    *self = Number::Leaf(0);
                    return Some((Some(left), Some(right)));
                }

                if let Some((add_left, add_right)) = left.explode(depth + 1) {
                    if let Some(add_right) = add_right {
                        right.add_to_leftmost(add_right);
                    }
                    return Some((add_left, None));
                }

                if let Some((add_left, add_right)) = right.explode(depth + 1) {
                    if let Some(add_left) = add_left {
                        left.add_to_rightmost(add_left);
                    }
                    return Some((None, add_right));
                }

                None
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Leaf(number) => {
                let number = *number;
                if number >= 10 {
                    let left = Number::Leaf(number / 2);
                    let right = Number::Leaf((number + 1) / 2);
                    *self = Number::Composite(Box::new((left, right)));

                    true
                } else {
                    false
                }
            }
            Number::Composite(b) => {
                let (left, right) = (&mut b.0, &mut b.1);
                left.split() || right.split()
            }
        }
    }

    fn add_to_rightmost(&mut self, number: u32) {
        match self {
            Number::Leaf(a) => *a += number,
            Number::Composite(b) => {
                b.1.add_to_rightmost(number);
            }
        }
    }

    fn add_to_leftmost(&mut self, number: u32) {
        match self {
            Number::Leaf(a) => *a += number,
            Number::Composite(b) => {
                b.0.add_to_leftmost(number);
            }
        }
    }

    fn add(self, other: Number) -> Number {
        let mut new = Number::Composite(Box::new((self, other)));

        loop {
            if new.explode(0).is_none() {
                if !new.split() {
                    return new;
                }
            }
        }
    }

    fn as_number(&self) -> u32 {
        match self {
            Number::Leaf(a) => *a,
            Number::Composite(_) => panic!(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number::Leaf(a) => *a,
            Number::Composite(b) => b.0.magnitude() * 3 + b.1.magnitude() * 2,
        }
    }
}

fn parse_input(input: String) -> Input {
    input
        .trim()
        .lines()
        .map(|line| Number::from_string(line).0)
        .collect()
}

fn task1(input: &Input) -> u32 {
    let input = input.clone();
    let result = input.into_iter().reduce(|a, b| a.add(b)).unwrap();
    result.magnitude()
}

fn task2(input: &Input) -> u32 {
    let mut max = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                let magnitude = input[i].clone().add(input[j].clone()).magnitude();
                max = max.max(magnitude);
            }
        }
    }

    max
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = parse_input(String::from(
        "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ",
    ));

    assert_eq!(task1(&input), 4140);
    assert_eq!(task2(&input), 3993);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 2501);
    assert_eq!(task2(&input), 4935);
}
