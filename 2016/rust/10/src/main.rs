use std::collections::HashMap;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

enum Output {
    Output(u32),
    Bot(u32),
}

enum Instruction {
    Input {
        value: u32,
        bot: u32,
    },
    Bot {
        bot: u32,
        low_output: Output,
        high_output: Output,
    },
}

type Input = Vec<Instruction>;

fn parse_input(input: &str) -> Input {
    let parse_output = |output: &str| {
        let output = output.trim();
        if let Some((_, number)) = output.split_once("output") {
            Output::Output(number.trim().parse().unwrap())
        } else if let Some((_, number)) = output.split_once("bot") {
            Output::Bot(number.trim().parse().unwrap())
        } else {
            panic!()
        }
    };

    input
        .trim()
        .split("\n")
        .map(|line| {
            if let Some((rhs, lhs)) = line.split_once("goes to bot") {
                let value = rhs.split_once("value").unwrap().1.trim().parse().unwrap();
                Instruction::Input {
                    value,
                    bot: lhs.trim().parse().unwrap(),
                }
            } else if let Some((rhs, lhs)) = line.split_once("gives low to") {
                let bot = rhs.split_once("bot").unwrap().1.trim().parse().unwrap();
                let (low_output, high_output) = lhs.split_once("and high to").unwrap();
                Instruction::Bot {
                    bot,
                    low_output: parse_output(low_output),
                    high_output: parse_output(high_output),
                }
            } else {
                panic!();
            }
        })
        .collect()
}

fn process(input: &Input) -> (HashMap<u32, u32>, HashMap<u32, (u32, Option<u32>)>) {
    let mut bots = HashMap::new();
    let mut todo = vec![];
    let mut outputs = HashMap::new();

    fn handle_output(
        bots: &mut HashMap<u32, (u32, Option<u32>)>,
        todo: &mut Vec<u32>,
        bot: u32,
        value: u32,
    ) {
        bots.entry(bot)
            .and_modify(|c| {
                if c.1.is_some() || c.0 == value {
                    panic!()
                }
                c.1 = Some(value);
                todo.push(bot);
            })
            .or_insert((value, None));
    }

    for instruction in input {
        if let Instruction::Input { value, bot } = instruction {
            handle_output(&mut bots, &mut todo, *bot, *value);
        }
    }

    while let Some(bot) = todo.pop() {
        for instruction in input {
            match instruction {
                Instruction::Bot {
                    bot: bot2,
                    low_output,
                    high_output,
                } if bot == *bot2 => {
                    let (value1, value2) = bots.get(&bot).unwrap();
                    let value2 = value2.unwrap();
                    let min = value2.min(*value1);
                    let max = value2.max(*value1);
                    match low_output {
                        Output::Output(output) => {
                            outputs.insert(*output, min);
                        }
                        Output::Bot(bot) => {
                            handle_output(&mut bots, &mut todo, *bot, min);
                        }
                    };
                    match high_output {
                        Output::Output(output) => {
                            outputs.insert(*output, max);
                        }
                        Output::Bot(bot) => {
                            handle_output(&mut bots, &mut todo, *bot, max);
                        }
                    };
                }
                _ => {}
            }
        }
    }

    (outputs, bots)
}

fn task1(input: &Input) -> u32 {
    let (_, bots) = process(input);
    for (bot, (chip1, chip2)) in bots {
        let chip2 = chip2.unwrap();
        let min = chip1.min(chip2);
        let max = chip1.max(chip2);
        if (min, max) == (17, 61) {
            return bot;
        }
    }

    unreachable!()
}

fn task2(input: &Input) -> u32 {
    let (outputs, _) = process(input);
    outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        value 5 goes to bot 2
        bot 2 gives low to bot 1 and high to bot 0
        value 3 goes to bot 1
        bot 1 gives low to output 1 and high to bot 0
        bot 0 gives low to output 2 and high to output 0
        value 2 goes to bot 2
        ";
    let input = parse_input(input);
    let (outputs, bots) = process(&input);

    assert_eq!(outputs.get(&0), Some(&5));
    assert_eq!(outputs.get(&1), Some(&2));
    assert_eq!(outputs.get(&2), Some(&3));
    assert_eq!(bots.get(&2), Some(&(5, Some(2))));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 141);
    assert_eq!(task2(&input), 1209);
}
