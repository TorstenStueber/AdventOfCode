use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Program {
    a: i64,
    b: i64,
    c: i64,
    instructions: Vec<u8>,
}

type Input = Program;

fn parse_input(input: &str) -> Input {
    let mut input = input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.split_once(':').unwrap().1.trim());

    let a = input.next().unwrap().parse().unwrap();
    let b = input.next().unwrap().parse().unwrap();
    let c = input.next().unwrap().parse().unwrap();

    let instructions = input
        .next()
        .unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    Program {
        a,
        b,
        c,
        instructions,
    }
}

fn task1(input: &Input) -> String {
    let mut instruction_pointer: usize = 0;
    let (mut a, mut b, mut c) = (input.a, input.b, input.c);
    let mut outputs = Vec::new();

    while instruction_pointer < input.instructions.len() - 1 {
        let opcode = input.instructions[instruction_pointer];
        let operand = input.instructions[instruction_pointer + 1];

        let combo_operand = match operand {
            0 | 1 | 2 | 3 => Some(operand as i64),
            4 => Some(a),
            5 => Some(b),
            6 => Some(c),
            _ => None,
        };

        instruction_pointer += 2;

        match opcode {
            0 => a = a / (1 << combo_operand.unwrap()),
            1 => b = b ^ operand as i64,
            2 => b = combo_operand.unwrap() % 8,
            3 => {
                if a != 0 {
                    instruction_pointer = operand as usize;
                }
            }
            4 => b = b ^ c,
            5 => outputs.push(format!("{}", combo_operand.unwrap() % 8)),
            6 => b = a / (1 << combo_operand.unwrap()),
            7 => c = a / (1 << combo_operand.unwrap()),
            _ => panic!(),
        }
    }

    outputs.join(",")
}

fn task2(input: &Input) -> i64 {
    fn solve_recursively(instructions: &[u8], a: i64) -> Option<i64> {
        if instructions.len() == 0 {
            return Some(a);
        }

        for candidate in 0..8 {
            let mut b = candidate;
            let previous_a = (a << 3) + candidate;
            b = b ^ 2;
            let c = (previous_a >> b) % 8;
            b = b ^ 7;
            b = b ^ c;

            if b == instructions[instructions.len() - 1] as i64 {
                if let Some(solution) =
                    solve_recursively(&instructions[..instructions.len() - 1], previous_a)
                {
                    return Some(solution);
                }
            }
        }
        None
    }

    solve_recursively(&input.instructions[..], 0).unwrap()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input1 = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0";

    let input1 = parse_input(test_input1);

    assert_eq!(task1(&input1), "4,6,3,5,6,3,5,2,1,0");
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), "3,1,5,3,7,4,2,7,5");
    assert_eq!(task2(&input), 190593310997519);
}
