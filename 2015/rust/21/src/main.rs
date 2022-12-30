use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug, Clone)]
struct Stats {
    hit_points: i32,
    damage: i32,
    armor: i32,
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
        armor: iter.next().unwrap(),
    }
}

fn human_wins(human: &Stats, robot: &Stats) -> bool {
    let mut human_hit_points = human.hit_points;
    let mut robot_hit_points = robot.hit_points;

    loop {
        robot_hit_points -= (human.damage - robot.armor).max(1);
        if robot_hit_points <= 0 {
            return true;
        }

        human_hit_points -= (robot.damage - human.armor).max(1);
        if human_hit_points <= 0 {
            return false;
        }
    }
}

struct Product {
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Product; 5] = [
    Product {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Product {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Product {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Product {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Product {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Product; 5] = [
    Product {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Product {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Product {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Product {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Product {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Product; 6] = [
    Product {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Product {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Product {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Product {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Product {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Product {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

fn task1(input: &Input) -> i32 {
    let mut min_cost = i32::MAX;

    for weapon in WEAPONS {
        for armor_index in 0..=ARMOR.len() {
            for first_ring_index in 0..=RINGS.len() {
                for second_ring_index in first_ring_index..=RINGS.len() {
                    if second_ring_index == first_ring_index && second_ring_index != RINGS.len() {
                        continue;
                    }

                    let mut human = Stats {
                        hit_points: 100,
                        damage: 0,
                        armor: 0,
                    };
                    human.armor = weapon.armor;
                    human.damage = weapon.damage;
                    let mut cost = weapon.cost;

                    if armor_index != ARMOR.len() {
                        human.armor += ARMOR[armor_index].armor;
                        human.damage += ARMOR[armor_index].damage;
                        cost += ARMOR[armor_index].cost;
                    }

                    if first_ring_index != RINGS.len() {
                        human.armor += RINGS[first_ring_index].armor;
                        human.damage += RINGS[first_ring_index].damage;
                        cost += RINGS[first_ring_index].cost;
                    }

                    if second_ring_index != RINGS.len() {
                        human.armor += RINGS[second_ring_index].armor;
                        human.damage += RINGS[second_ring_index].damage;
                        cost += RINGS[second_ring_index].cost;
                    }

                    if cost < min_cost && human_wins(&human, input) {
                        min_cost = cost;
                    }
                }
            }
        }
    }

    min_cost
}

fn task2(input: &Input) -> i32 {
    let mut max_cost = i32::MIN;

    for weapon in WEAPONS {
        for armor_index in 0..=ARMOR.len() {
            for first_ring_index in 0..=RINGS.len() {
                for second_ring_index in first_ring_index..=RINGS.len() {
                    if second_ring_index == first_ring_index && second_ring_index != RINGS.len() {
                        continue;
                    }

                    let mut human = Stats {
                        hit_points: 100,
                        damage: 0,
                        armor: 0,
                    };
                    human.armor = weapon.armor;
                    human.damage = weapon.damage;
                    let mut cost = weapon.cost;

                    if armor_index != ARMOR.len() {
                        human.armor += ARMOR[armor_index].armor;
                        human.damage += ARMOR[armor_index].damage;
                        cost += ARMOR[armor_index].cost;
                    }

                    if first_ring_index != RINGS.len() {
                        human.armor += RINGS[first_ring_index].armor;
                        human.damage += RINGS[first_ring_index].damage;
                        cost += RINGS[first_ring_index].cost;
                    }

                    if second_ring_index != RINGS.len() {
                        human.armor += RINGS[second_ring_index].armor;
                        human.damage += RINGS[second_ring_index].damage;
                        cost += RINGS[second_ring_index].cost;
                    }

                    if cost > max_cost && !human_wins(&human, input) {
                        max_cost = cost;
                    }
                }
            }
        }
    }

    max_cost
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let human_test_input = "
        Hit Points: 8
        Damage: 5
        Armor: 5
        ";

    let robot_test_input = "
        Hit Points: 12
        Damage: 7
        Armor: 2
        ";

    let human_input = parse_input(&human_test_input);
    let robot_input = parse_input(&robot_test_input);
    assert_eq!(human_wins(&human_input, &robot_input), true);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 91);
    assert_eq!(task2(&input), 158);
}
