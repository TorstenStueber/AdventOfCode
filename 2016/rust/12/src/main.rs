use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    CpyValue(i32, Register),
    CpyRegister(Register, Register),
    Inc(Register),
    Dec(Register),
    JnzValue(i32, isize),
    JnzRegister(Register, isize),
}

type Input = Vec<Instruction>;

fn parse_input(input: &str) -> Input {
    fn parse_register(s: &str) -> Option<Register> {
        match s.trim() {
            "a" => Some(Register::A),
            "b" => Some(Register::B),
            "c" => Some(Register::C),
            "d" => Some(Register::D),
            _ => None,
        }
    }
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim();
            if line.starts_with("cpy") {
                let (x, y) = line[4..].split_once(" ").unwrap();
                if let Some(register) = parse_register(x) {
                    Instruction::CpyRegister(register, parse_register(y).unwrap())
                } else {
                    Instruction::CpyValue(x.trim().parse().unwrap(), parse_register(y).unwrap())
                }
            } else if line.starts_with("inc") {
                Instruction::Inc(parse_register(&line[4..]).unwrap())
            } else if line.starts_with("dec") {
                Instruction::Dec(parse_register(&line[4..]).unwrap())
            } else if line.starts_with("jnz") {
                let (x, y) = line[4..].split_once(" ").unwrap();
                if let Some(register) = parse_register(x) {
                    Instruction::JnzRegister(register, y.trim().parse().unwrap())
                } else {
                    Instruction::JnzValue(x.trim().parse().unwrap(), y.trim().parse().unwrap())
                }
            } else {
                panic!()
            }
        })
        .collect()
}

#[derive(Default)]
struct State {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    ip: usize,
}

fn execute(input: &Input, mut state: State) -> i32 {
    fn get_register(state: &mut State, register: Register) -> &mut i32 {
        match register {
            Register::A => &mut state.a,
            Register::B => &mut state.b,
            Register::C => &mut state.c,
            Register::D => &mut state.d,
        }
    }

    while state.ip < input.len() {
        match input[state.ip] {
            Instruction::CpyValue(value, register) => *get_register(&mut state, register) = value,
            Instruction::CpyRegister(source, target) => {
                *get_register(&mut state, target) = *get_register(&mut state, source)
            }
            Instruction::Inc(register) => *get_register(&mut state, register) += 1,
            Instruction::Dec(register) => *get_register(&mut state, register) -= 1,
            Instruction::JnzValue(value, offset) => {
                if value != 0 {
                    state.ip = (state.ip as isize + offset - 1) as usize
                }
            }
            Instruction::JnzRegister(register, offset) => {
                if *get_register(&mut state, register) != 0 {
                    state.ip = (state.ip as isize + offset - 1) as usize
                }
            }
        }
        state.ip += 1;
    }

    state.a
}

fn task1(input: &Input) -> i32 {
    execute(input, Default::default())
}

fn task2(input: &Input) -> i32 {
    execute(
        input,
        State {
            c: 1,
            ..Default::default()
        },
    )
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
        cpy 41 a
        inc a
        inc a
        dec a
        jnz a 2
        dec a
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), 42);
}

#[test]
fn task() {
    let raw_input = load_input();
    let input = parse_input(&raw_input);
    assert_eq!(task1(&input), 317993);
    assert_eq!(task2(&input), 9227647);
}
