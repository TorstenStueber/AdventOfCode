use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
enum Instruction {
    HalfA,
    HalfB,
    TripleA,
    TripleB,
    IncA,
    IncB,
    Jmp(i32),
    JumpEvenA(i32),
    JumpEvenB(i32),
    JumpOneA(i32),
    JumpOneB(i32),
}

type Input = Vec<Instruction>;

use Instruction::*;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim();
            match line {
                "hlf a" => HalfA,
                "hlf b" => HalfB,
                "tpl a" => TripleA,
                "tpl b" => TripleB,
                "inc a" => IncA,
                "inc b" => IncB,
                _ => {
                    if let Some((_, offset)) = line.split_once("jmp") {
                        Jmp(offset.trim().parse().unwrap())
                    } else if let Some((_, offset)) = line.split_once("jie a,") {
                        JumpEvenA(offset.trim().parse().unwrap())
                    } else if let Some((_, offset)) = line.split_once("jie b,") {
                        JumpEvenB(offset.trim().parse().unwrap())
                    } else if let Some((_, offset)) = line.split_once("jio a,") {
                        JumpOneA(offset.trim().parse().unwrap())
                    } else if let Some((_, offset)) = line.split_once("jio b,") {
                        JumpOneB(offset.trim().parse().unwrap())
                    } else {
                        panic!()
                    }
                }
            }
        })
        .collect()
}

fn run(input: &Input, mut a: u32) -> u32 {
    let mut ip = 0i32;
    let mut b = 0u32;
    while ip >= 0 && ip < input.len() as i32 {
        match input[ip as usize] {
            HalfA => a >>= 1,
            HalfB => b >>= 1,
            TripleA => a *= 3,
            TripleB => b *= 3,
            IncA => a += 1,
            IncB => b += 1,
            Jmp(offset) => ip += offset - 1,
            JumpEvenA(offset) => {
                if a % 2 == 0 {
                    ip += offset - 1
                }
            }
            JumpEvenB(offset) => {
                if b % 2 == 0 {
                    ip += offset - 1
                }
            }
            JumpOneA(offset) => {
                if a == 1 {
                    ip += offset - 1
                }
            }
            JumpOneB(offset) => {
                if b == 1 {
                    ip += offset - 1
                }
            }
        }
        ip += 1;
    }

    b
}

fn task1(input: &Input) -> u32 {
    run(input, 0)
}

fn task2(input: &Input) -> u32 {
    run(input, 1)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = "
        inc b
        jio b, +2
        tpl b
        inc b
        ";

    let input = parse_input(&test_input);
    assert_eq!(task1(&input), 2);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 307);
    assert_eq!(task2(&input), 160);
}
