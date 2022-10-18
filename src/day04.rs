use std::str::Split;
use crate::read_input;

#[derive(Clone, Copy)]
struct Board {
    board: [[i32; 5]; 5],
    mark: [[bool; 5]; 5],
}

fn get_input() -> String {
    let input = read_input::read("inputs/04.txt");
    input
}

fn add_string_to_board(
    row_index: usize,
    col_index: usize,
    board: &mut Board,
    num_string: String) {
    board.board[row_index][col_index] = num_string
        .trim()
        .parse::<i32>()
        .expect("Value wasn't an integer.");
}

fn construct_board(line_set: [String; 5]) -> Board {
    let mut board: Board = Board {
        board: [[0; 5]; 5],
        mark: [[false; 5]; 5],
    };
    for (row_index, line) in line_set.iter().enumerate() {
        let mut col_index = 0;
        let mut num_string = String::new();
        for char in line.chars() {
            if char != ' ' {
                num_string.push(char);
            } else {
                if num_string.len() != 0 {
                    add_string_to_board(row_index, col_index, &mut board, num_string);
                    num_string = String::new();
                    col_index += 1;
                }
            }
        }
        // last column isn't noticed by above condition (line doesn't end in ' ')
        add_string_to_board(row_index, col_index, &mut board, num_string);
    }
    board
}

fn print_board(board: Board) {
    for (row_index, row) in board.board.iter().enumerate() {
        let mut row_string = String::new();

        for (col_index, col) in row.iter().enumerate() {
            if board.mark[row_index][col_index] {
                row_string.push_str(" X");
            } else {
                if format!("{col}").as_str().len() == 1 {
                    row_string.push_str(" ");
                    row_string.push_str(format!("{col}").as_str());
                } else {
                    row_string.push_str(format!("{col}").as_str());
                }
            }
            row_string.push_str(" ");
        }
        row_string.push_str("  |  ");
        for col in row {
            if format!("{col}").as_str().len() == 1 {
                row_string.push_str(" ");
                row_string.push_str(format!("{col}").as_str());
            } else {
                row_string.push_str(format!("{col}").as_str());
            }
            row_string.push_str(" ");
        }
        println!("{}", row_string);
    }
    println!();
}

fn get_instruction_list(instruction_string: String) -> Vec<i32> {
    let mut instructions = Vec::new();
    for num_string in instruction_string.split(",") {
        instructions.push(
            num_string
                .trim()
                .parse::<i32>()
                .expect("Value wasn't an integer")
        )
    }
    instructions
}

fn get_board_list(remaining_split: &mut Split<&str>) -> Vec<Board> {
    let mut board_list = Vec::new();
    const DUMMY_STRING: String = String::new();
    let mut board_string = [DUMMY_STRING; 5];
    while remaining_split.next().is_some() {
        for i in 0..5 {
            match remaining_split.next() {
                Some(string) => board_string[i] = String::from(string),
                None => continue,
            }
            if i == 4 {
                board_list.push(construct_board(board_string.clone()));
            }
        }
    }
    board_list
}

fn get_instructions_and_boards() -> (Vec<i32>, Vec<Board>) {

    let input = get_input();
    let mut lines = input.split("\n");
    let instruction_string: String;
    match lines.next() {
        Some(string) => instruction_string = String::from(string),
        None => panic!("No instructions found"),
    }
    let instruction_list = get_instruction_list(instruction_string);
    let board_list = get_board_list(&mut lines);

    (instruction_list, board_list)
}

fn mark_val_on_boards(val: i32, boards: &mut Vec<Board>) {
    for board in boards {
        for (row_index, row) in board.board.iter().enumerate() {
            for (col_index, entry) in row.iter().enumerate() {
                if *entry == val {
                    board.mark[row_index][col_index] = true;
                }
            }
        }
    }
}

fn board_has_won(board: &Board) -> bool {
    // rows
    for row in board.mark {
        let mut row_won: bool = true;
        for entry in row {
            if !entry {
                row_won = false;
            }
        }
        if row_won {
            return true;
        }
    }
    // cols
    for col_index in 0..5 {
        let mut col_won = true;
        for row in board.mark {
            if !row[col_index] {
                col_won = false;
            }
        }
        if col_won {
            return true;
        }
    }
    false
}

fn get_sum_of_unmarked_values(board: &Board) -> i32 {
    let mut sum: i32 = 0;
    for (row_index, row) in board.mark.iter().enumerate() {
        for (col_index, entry) in row.iter().enumerate() {
            if !entry {
                sum += board.board[row_index][col_index]
            }
        }
    }
    sum
}

fn part_1() -> i32 {
    let (instructions, mut board_list) = get_instructions_and_boards();
    for instr in instructions {
        mark_val_on_boards(instr, &mut board_list);
        for board in board_list.iter() {
            if board_has_won(board) {
                let board_sum = get_sum_of_unmarked_values(&board);
                return board_sum * instr;
            }
        }
    }
    panic!("No winning board discovered.")
}

fn part_2() -> i32 {
    let (instructions, mut prev_unfinished_boards) = get_instructions_and_boards();
    for (i, instr) in instructions.iter().enumerate() {
        println!("{i}: {}", *instr);
        let mut new_unfinished_boards = Vec::new();
        mark_val_on_boards(*instr, &mut prev_unfinished_boards);
        for board in prev_unfinished_boards.iter() {
            if !board_has_won(board) {
                new_unfinished_boards.push(board.clone())
            } else {
                println!("Eliminated board")
            }
        }

        if new_unfinished_boards.len() == 0 {
            println!("All boards have won");
            let prev_b_len = prev_unfinished_boards.len();
            println!("Length of prev boards is {}", prev_b_len);
            let last_board = match prev_unfinished_boards.pop() {
                Some(b) => b,
                None => panic!("No last board - what?"),
            };

            let last_board_sum = get_sum_of_unmarked_values(&last_board);
            println!("Winning value: {}", instr);
            println!("Winning board sum: {}", last_board_sum);
            print_board(last_board);
            return last_board_sum * instr;
        }

        prev_unfinished_boards = new_unfinished_boards

    }
    panic!("There was never only one winning board.")
}

pub fn solve_puzzle() {
    println!("Day 4");
    // let ans_1 = part_1();
    // println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}