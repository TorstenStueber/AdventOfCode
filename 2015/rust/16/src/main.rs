use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
struct Aunt {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

type Input = Vec<Aunt>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let mut aunt = Aunt {
                children: None,
                cats: None,
                samoyeds: None,
                pomeranians: None,
                akitas: None,
                vizslas: None,
                goldfish: None,
                trees: None,
                cars: None,
                perfumes: None,
            };
            line.split(",").for_each(|component| {
                let (name, count) = component.split_once(":").unwrap();
                let count = count.trim().parse().unwrap();
                let name = name.trim();
                match name {
                    "children" => aunt.children = Some(count),
                    "cats" => aunt.cats = Some(count),
                    "samoyeds" => aunt.samoyeds = Some(count),
                    "pomeranians" => aunt.pomeranians = Some(count),
                    "akitas" => aunt.akitas = Some(count),
                    "vizslas" => aunt.vizslas = Some(count),
                    "goldfish" => aunt.goldfish = Some(count),
                    "trees" => aunt.trees = Some(count),
                    "cars" => aunt.cars = Some(count),
                    "perfumes" => aunt.perfumes = Some(count),
                    _ => panic!(),
                }
            });
            aunt
        })
        .collect()
}

fn task1(input: &Input) -> usize {
    let children = 3;
    let cats = 7;
    let samoyeds = 2;
    let pomeranians = 3;
    let akitas = 0;
    let vizslas = 0;
    let goldfish = 5;
    let trees = 3;
    let cars = 2;
    let perfumes = 1;

    for (index, aunt) in input.iter().enumerate() {
        let same = aunt.children.map_or(true, |x| x == children)
            && aunt.cats.map_or(true, |x| x == cats)
            && aunt.samoyeds.map_or(true, |x| x == samoyeds)
            && aunt.pomeranians.map_or(true, |x| x == pomeranians)
            && aunt.akitas.map_or(true, |x| x == akitas)
            && aunt.vizslas.map_or(true, |x| x == vizslas)
            && aunt.goldfish.map_or(true, |x| x == goldfish)
            && aunt.trees.map_or(true, |x| x == trees)
            && aunt.cars.map_or(true, |x| x == cars)
            && aunt.perfumes.map_or(true, |x| x == perfumes);

        if same {
            return index + 1;
        }
    }

    unreachable!()
}

fn task2(input: &Input) -> usize {
    let children = 3;
    let cats = 7;
    let samoyeds = 2;
    let pomeranians = 3;
    let akitas = 0;
    let vizslas = 0;
    let goldfish = 5;
    let trees = 3;
    let cars = 2;
    let perfumes = 1;

    for (index, aunt) in input.iter().enumerate() {
        let same = aunt.children.map_or(true, |x| x == children)
            && aunt.cats.map_or(true, |x| x > cats)
            && aunt.samoyeds.map_or(true, |x| x == samoyeds)
            && aunt.pomeranians.map_or(true, |x| x < pomeranians)
            && aunt.akitas.map_or(true, |x| x == akitas)
            && aunt.vizslas.map_or(true, |x| x == vizslas)
            && aunt.goldfish.map_or(true, |x| x < goldfish)
            && aunt.trees.map_or(true, |x| x > trees)
            && aunt.cars.map_or(true, |x| x == cars)
            && aunt.perfumes.map_or(true, |x| x == perfumes);

        if same {
            return index + 1;
        }
    }

    unreachable!()
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1(&input), 213);
    assert_eq!(task2(&input), 323);
}
