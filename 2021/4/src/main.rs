use std::fs;

fn load_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Clone, Debug, Copy)]
struct BingoField {
    number: u32,
    drawn: bool,
}

struct Input {
    numbers: Vec<u32>,
    boards: Vec<[[BingoField; 5]; 5]>,
}

fn parse_input(input: String) -> Input {
    let (numbers, boards_string) = input.trim().split_once("\n").unwrap();

    let numbers: Vec<u32> = numbers
        .trim()
        .split(",")
        .map(|number| number.trim().parse().unwrap())
        .collect();

    let mut boards: Vec<[[BingoField; 5]; 5]> = Vec::new();

    let mut row_number = 0;
    let mut board = [[BingoField {
        number: 0,
        drawn: false,
    }; 5]; 5];

    for line in boards_string.trim().lines() {
        if row_number == 5 {
            boards.push(board.clone());
            row_number = 0;
            assert!(line.trim().len() == 0);
            continue;
        }

        for (column_number, number_string) in line.trim().split_whitespace().enumerate() {
            board[row_number][column_number] = BingoField {
                number: number_string.trim().parse().unwrap(),
                drawn: false,
            }
        }
        row_number += 1;
    }

    if row_number == 5 {
        boards.push(board.clone());
    }

    Input { numbers, boards }
}

fn did_board_win(board: &[[BingoField; 5]; 5]) -> bool {
    for i in 0..5 {
        if (0..5).all(|x| board[x][i].drawn) {
            return true;
        }

        if (0..5).all(|y| board[i][y].drawn) {
            return true;
        }
    }

    false
}

fn determine_board_score(board: &[[BingoField; 5]; 5]) -> u32 {
    let mut sum = 0;
    for line in board.iter() {
        for field in line.iter() {
            if !field.drawn {
                sum += field.number;
            }
        }
    }

    sum
}

fn task1(input: &Input) -> u32 {
    let mut boards = input.boards.clone();

    for drawn_number in input.numbers.iter() {
        for board in boards.iter_mut() {
            let mut found = false;
            'outer: for line in board.iter_mut() {
                for field in line.iter_mut() {
                    if field.number == *drawn_number {
                        field.drawn = true;
                        found = true;
                        break 'outer;
                    }
                }
            }

            if found {
                if did_board_win(board) {
                    return determine_board_score(board) * drawn_number;
                }
            }
        }
    }

    unreachable!();
}

fn task2(input: &Input) -> u32 {
    let mut boards = input.boards.clone();

    for drawn_number in input.numbers.iter() {
        for board in boards.iter_mut() {
            'outer: for line in board.iter_mut() {
                for field in line.iter_mut() {
                    if field.number == *drawn_number {
                        field.drawn = true;
                        break 'outer;
                    }
                }
            }
        }

        let last_board = boards[0].clone();

        let remaining_boards: Vec<[[BingoField; 5]; 5]> = boards
            .into_iter()
            .filter(|board| !did_board_win(board))
            .collect();

        if remaining_boards.len() == 0 {
            return determine_board_score(&last_board) * drawn_number;
        }

        boards = remaining_boards;
    }

    unreachable!();
}

fn main() {
    let input = parse_input(load_input());
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
}

#[test]
fn example() {
    let test_input = String::from(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7",
    );

    let input = parse_input(test_input);
    assert_eq!(task1(&input), 4512);
    assert_eq!(task2(&input), 1924);
}

#[test]
fn task() {
    let input = parse_input(load_input());
    assert_eq!(task1(&input), 35670);
    assert_eq!(task2(&input), 22704);
}
