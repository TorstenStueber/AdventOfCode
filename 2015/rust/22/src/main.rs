use std::collections::HashSet;
use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Stats {
    hit_points: i32,
    damage: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    player_hit_points: i32,
    boss_hit_points: i32,
    mana: i32,
    shield_active: Option<u8>,
    poison_active: Option<u8>,
    recharge_active: Option<u8>,
    used_mana: i32,
}

type Input = Stats;

fn parse_input(input: &str) -> Input {
    let mut iter = input
        .trim()
        .split("\n")
        .map(|line| line.split_once(":").unwrap().1.trim().parse().unwrap());

    Stats {
        hit_points: iter.next().unwrap(),
        damage: iter.next().unwrap(),
    }
}

fn apply_effects(state: &mut State) {
    if let Some(rounds_left) = state.recharge_active {
        state.mana += 101;
        state.recharge_active = match rounds_left {
            1 => None,
            _ => Some(rounds_left - 1),
        };
    }

    if let Some(rounds_left) = state.poison_active {
        state.boss_hit_points -= 3;
        state.poison_active = match rounds_left {
            1 => None,
            _ => Some(rounds_left - 1),
        };
    }

    if let Some(rounds_left) = state.shield_active {
        state.shield_active = match rounds_left {
            1 => None,
            _ => Some(rounds_left - 1),
        };
    }
}

fn double_step_1(boss: &Stats, mut new_state: State, minimum_mana: &mut i32) -> Option<State> {
    fn update_minimum_mana(state: &State, minimum_mana: &mut i32) {
        if state.used_mana < *minimum_mana {
            *minimum_mana = state.used_mana;
        }
    }

    apply_effects(&mut new_state);
    if new_state.boss_hit_points <= 0 {
        update_minimum_mana(&new_state, minimum_mana);
        return None;
    }

    new_state.player_hit_points -= 1.max(
        boss.damage
            - if new_state.shield_active.is_some() {
                7
            } else {
                0
            },
    );

    if new_state.player_hit_points <= 0 {
        return None;
    }

    apply_effects(&mut new_state);
    if new_state.boss_hit_points <= 0 {
        update_minimum_mana(&new_state, minimum_mana);
        return None;
    }

    Some(new_state)
}

fn double_step_2(boss: &Stats, mut new_state: State, minimum_mana: &mut i32) -> Option<State> {
    new_state.player_hit_points -= 1;
    if new_state.player_hit_points <= 0 {
        return None;
    }

    double_step_1(boss, new_state, minimum_mana)
}

fn iterate(input: &Input, double_step: fn(&Stats, State, &mut i32) -> Option<State>) -> i32 {
    let mut seen_states = HashSet::new();
    let mut todo = vec![State {
        player_hit_points: 50,
        boss_hit_points: input.hit_points,
        mana: 500,
        shield_active: None,
        poison_active: None,
        recharge_active: None,
        used_mana: 0,
    }];

    fn add_maybe_new_state(
        seen_states: &mut HashSet<State>,
        todo: &mut Vec<State>,
        maybe_new_state: Option<State>,
    ) {
        if let Some(new_state) = maybe_new_state {
            if !seen_states.contains(&new_state) {
                seen_states.insert(new_state.clone());
                todo.push(new_state);
            }
        }
    }

    let mut minimum_used_mana = i32::MAX;

    while let Some(state) = todo.pop() {
        if state.mana >= 53 {
            let mut new_state = state.clone();
            new_state.mana -= 53;
            new_state.used_mana += 53;
            new_state.boss_hit_points -= 4;
            let maybe_new_state = double_step(input, new_state, &mut minimum_used_mana);
            add_maybe_new_state(&mut seen_states, &mut todo, maybe_new_state);
        }

        if state.mana >= 73 {
            let mut new_state = state.clone();
            new_state.mana -= 73;
            new_state.used_mana += 73;
            new_state.boss_hit_points -= 2;
            new_state.player_hit_points += 2;
            let maybe_new_state = double_step(input, new_state, &mut minimum_used_mana);
            add_maybe_new_state(&mut seen_states, &mut todo, maybe_new_state);
        }

        if state.mana >= 113 && state.shield_active.is_none() {
            let mut new_state = state.clone();
            new_state.mana -= 113;
            new_state.used_mana += 113;
            new_state.shield_active = Some(6);
            let maybe_new_state = double_step(input, new_state, &mut minimum_used_mana);
            add_maybe_new_state(&mut seen_states, &mut todo, maybe_new_state);
        }

        if state.mana >= 173 && state.poison_active.is_none() {
            let mut new_state = state.clone();
            new_state.mana -= 173;
            new_state.used_mana += 173;
            new_state.poison_active = Some(6);
            let maybe_new_state = double_step(input, new_state, &mut minimum_used_mana);
            add_maybe_new_state(&mut seen_states, &mut todo, maybe_new_state);
        }

        if state.mana >= 229 && state.recharge_active.is_none() {
            let mut new_state = state.clone();
            new_state.mana -= 229;
            new_state.used_mana += 229;
            new_state.recharge_active = Some(5);
            let maybe_new_state = double_step(input, new_state, &mut minimum_used_mana);
            add_maybe_new_state(&mut seen_states, &mut todo, maybe_new_state);
        }
    }

    minimum_used_mana
}

fn task1(input: &Input) -> i32 {
    iterate(input, double_step_1)
}

fn task2(input: &Input) -> i32 {
    iterate(input, double_step_2)
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 1269);
    assert_eq!(task2(&input), 1309);
}
