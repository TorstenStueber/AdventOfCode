use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = Vec<(Vec<String>, Vec<String>)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim();
            let mut supernet_sequences: Vec<String> = vec![];
            let mut hypernet_sequences: Vec<String> = vec![];
            let mut cursor = 0;
            while let Some(mut position) = line[cursor..].find('[') {
                position += cursor;
                supernet_sequences.push(line[cursor..position].into());
                cursor = line[position..].find(']').unwrap() + 1 + position;
                hypernet_sequences.push(line[position + 1..cursor - 1].into());
            }
            supernet_sequences.push(line[cursor..].into());

            (supernet_sequences, hypernet_sequences)
        })
        .collect::<Vec<_>>()
}

fn task1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(supernet_sequences, hypernet_sequences)| {
            if hypernet_sequences.iter().any(|string| {
                string
                    .as_bytes()
                    .windows(4)
                    .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
            }) {
                return false;
            }

            supernet_sequences.iter().any(|string| {
                string
                    .as_bytes()
                    .windows(4)
                    .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
            })
        })
        .count()
}

fn task2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(supernet_sequences, hypernet_sequences)| {
            let abas: Vec<_> = supernet_sequences
                .iter()
                .map(|string| {
                    string
                        .as_bytes()
                        .windows(3)
                        .filter(|w| w[0] == w[2] && w[0] != w[1])
                        .map(|w| (w[0], w[1]))
                })
                .flatten()
                .collect();

            let babs: Vec<_> = hypernet_sequences
                .iter()
                .map(|string| {
                    string
                        .as_bytes()
                        .windows(3)
                        .filter(|w| w[0] == w[2] && w[0] != w[1])
                        .map(|w| (w[1], w[0]))
                })
                .flatten()
                .collect();

            abas.iter().any(|aba| babs.iter().any(|bab| bab == aba))
        })
        .count()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let input = "
        abba[mnop]qrst
        abcd[bddb]xyyx
        aaaa[qwer]tyui
        ioxxoj[asdfgh]zxcvbn
        ";
    let input = parse_input(input);

    assert_eq!(task1(&input), 2);

    let input = "
        aba[bab]xyz
        xyx[xyx]xyx
        aaa[kek]eke
        zazbz[bzb]cdb
        ";
    let input = parse_input(input);

    assert_eq!(task2(&input), 3);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 115);
    assert_eq!(task2(&input), 231);
}
