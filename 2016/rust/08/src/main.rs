use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, isize),
    RotateCol(usize, isize),
}

type Input = Vec<Instruction>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim();

            if let Some((_, rhs)) = line.split_once("rect") {
                let (w, h) = rhs.trim().split_once("x").unwrap();
                Instruction::Rect(w.trim().parse().unwrap(), h.trim().parse().unwrap())
            } else if let Some((_, rhs)) = line.split_once("row y=") {
                let (y, n) = rhs.trim().split_once("by").unwrap();
                Instruction::RotateRow(y.trim().parse().unwrap(), n.trim().parse().unwrap())
            } else if let Some((_, rhs)) = line.split_once("column x=") {
                let (x, n) = rhs.trim().split_once("by").unwrap();
                Instruction::RotateCol(x.trim().parse().unwrap(), n.trim().parse().unwrap())
            } else {
                panic!()
            }
        })
        .collect::<Vec<_>>()
}

fn compute<const WIDTH: usize, const HEIGHT: usize>(input: &Input) -> [[bool; HEIGHT]; WIDTH] {
    let mut screen = [[false; HEIGHT]; WIDTH];
    for instruction in input {
        match instruction {
            Instruction::Rect(w, h) => {
                for x in 0..*w {
                    for y in 0..*h {
                        screen[x][y] = true;
                    }
                }
            }
            Instruction::RotateRow(y, n) => {
                let row: Vec<_> = screen.iter().map(|column| column[*y]).collect();
                for x in 0..WIDTH {
                    let width = WIDTH as isize;
                    screen[x][*y] = row[(((x as isize - n) % width + width) % width) as usize];
                }
            }
            Instruction::RotateCol(x, n) => {
                let column = screen[*x];
                for y in 0..HEIGHT {
                    let height = HEIGHT as isize;
                    screen[*x][y] =
                        column[(((y as isize - n) % height + height) % height) as usize];
                }
            }
        }
    }
    screen
}

fn task1<const WIDTH: usize, const HEIGHT: usize>(input: &Input) -> usize {
    let screen = compute::<WIDTH, HEIGHT>(input);

    screen.iter().fold(0, |acc, column| {
        acc + column.iter().fold(0, |a, b| a + if *b { 1 } else { 0 })
    })
}

fn task2<const WIDTH: usize, const HEIGHT: usize>(input: &Input) -> String {
    let screen = compute::<WIDTH, HEIGHT>(input);
    (0..HEIGHT)
        .map(|y| (0..WIDTH).map(|x| screen[x][y]).collect::<Vec<_>>())
        .map(|row| {
            row.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let input = parse_input(&load_input());
    println!("Task 1: {}", task1::<50, 6>(&input));
    println!("Task 2:\n{}", task2::<50, 6>(&input));
}

#[test]
fn example() {
    let input = "
        rect 3x2
        rotate column x=1 by 1
        rotate row y=0 by 4
        rotate column x=1 by 1
        ";
    let input = parse_input(input);

    assert_eq!(task1::<7, 3>(&input), 6);

    let result = ".#..#.#\n#.#....\n.#.....";
    assert_eq!(task2::<7, 3>(&input), result);
}

#[test]
fn task() {
    let input = parse_input(&load_input());
    assert_eq!(task1::<50, 6>(&input), 119);

    let result2 = "####.####.#..#.####..###.####..##...##..###...##..
...#.#....#..#.#....#....#....#..#.#..#.#..#.#..#.
..#..###..####.###..#....###..#..#.#....#..#.#..#.
.#...#....#..#.#.....##..#....#..#.#.##.###..#..#.
#....#....#..#.#.......#.#....#..#.#..#.#....#..#.
####.#....#..#.#....###..#.....##...###.#.....##..";
    assert_eq!(task2::<50, 6>(&input), result2);
}
