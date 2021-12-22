use std::{collections::HashMap, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

type Input = (u32, u32);

fn parse_input(input: String) -> Input {
    let (line1, line2) = input.trim().split_once("\n").unwrap();
    (
        line1.trim().split_once(": ").unwrap().1.parse().unwrap(),
        line2.trim().split_once(": ").unwrap().1.parse().unwrap(),
    )
}

fn task1(input: &Input) -> u32 {
    let mut die = 0;
    let mut pawns = vec![input.0 - 1, input.1 - 1];
    let mut points = vec![0, 0];
    let mut player = 0;

    while points[0] < 1000 && points[1] < 1000 {
        pawns[player] += (die % 100) + (die + 1) % 100 + (die + 2) % 100 + 3;
        die += 3;
        points[player] += pawns[player] % 10 + 1;

        player = 1 - player;
    }

    die * points[player]
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct GameState {
    pawns: [u8; 2],
    scores: [u8; 2],
}

fn roll_die(state: GameState, player: usize) -> Vec<GameState> {
    if state.scores[0] >= 21 || state.scores[1] >= 21 {
        return vec![state];
    }

    let mut result = vec![state.clone(), state.clone(), state];
    for (index, state) in result.iter_mut().enumerate() {
        state.pawns[player] += index as u8 + 1;
        if state.pawns[player] > 10 {
            state.pawns[player] -= 10
        }
    }

    result
}

fn add_score(mut state: GameState, player: usize) -> Vec<GameState> {
    if state.scores[0] >= 21 || state.scores[1] >= 21 {
        return vec![state];
    }

    state.scores[player] += state.pawns[player];

    vec![state]
}

fn apply_change<T: Fn(GameState, usize) -> Vec<GameState>>(
    state_counter: HashMap<GameState, u64>,
    updater: T,
    player: usize,
) -> (HashMap<GameState, u64>, bool) {
    let mut new_state_counter = HashMap::new();

    let mut all_won = true;
    for (state, counts) in state_counter {
        for new_state in updater(state, player) {
            if new_state.scores[0] < 21 && new_state.scores[1] < 21 {
                all_won = false;
            }
            *new_state_counter.entry(new_state).or_insert(0) += counts;
        }
    }

    (new_state_counter, all_won)
}

fn task2(input: &Input) -> u64 {
    let initial_state = GameState {
        pawns: [input.0 as u8, input.1 as u8],
        scores: [0, 0],
    };

    let mut state_counter = HashMap::new();
    state_counter.insert(initial_state, 1);
    let mut player = 0;
    loop {
        for _ in 0..3 {
            state_counter = apply_change(state_counter, roll_die, player).0;
        }
        let changes = apply_change(state_counter, add_score, player);
        state_counter = changes.0;
        if changes.1 {
            break;
        }
        player = 1 - player;
    }

    let mut count1 = 0;
    let mut count2 = 0;
    for (state, counts) in state_counter {
        if state.scores[0] >= 21 {
            count1 += counts;
        } else {
            count2 += counts;
        }
    }

    count1.max(count2)
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
        Player 1 starting position: 4
        Player 2 starting position: 8
        ",
    ));

    assert_eq!(task1(&input), 739785);
    assert_eq!(task2(&input), 444356092776315);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 913560);
    assert_eq!(task2(&input), 110271560863819);
}
