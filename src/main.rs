use std::io::{stdin, stdout, Write};

pub mod board;
pub mod boardvalue;
pub mod minimax;

use crate::board::*;
use crate::boardvalue::*;
use crate::minimax::*;

///asks the player for a new move using stdin, returns a grid index
fn get_player_move() -> i32 {
    let mut user_input_buffer = String::new();
    print!("Please enter a number that corresponds to a cell");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut user_input_buffer)
        .expect("Not a valid input");

    user_input_buffer.trim().parse::<i32>().unwrap()
}

//takes a copy of the board, and returns a new board with it's best move
fn get_computer_move(board: Board) -> Board {
    //get all possible children for the maximizing player 
    let children = board.get_children(true);
    
    //set up the temporary variables to store the future max value and best move
    let mut max = i32::MIN;
    let mut best_move = Board::new();

    //every "child" here is a possible move
    for child in children {

        let score = minimax(child.clone(), false);
        if score > max {
            max = score;
            best_move = child;
        }
    }
    best_move
}

fn main() {
    let mut board = Board::new();

    //print the indices that the user can place O's on
    println!("\n_____________\n\n| 0 | 1 | 2 |\n_____________\n\n| 3 | 4 | 5 |\n_____________\n\n| 6 | 7 | 8 |\n_____________\n");

    //loop as long as the board is not in a terminal state
    while !board.is_terminal() {

        //computer player first
        let computer_move = get_computer_move(board.clone());
        board = computer_move;
        if board.is_terminal() {
            break;
        }

        //show board before asking player for input
        println!("{}", board);

        let player_move = get_player_move() as usize;
        //take the input, and place an O on that grid cell
        board.grid[player_move] = BoardValue::O;
    }
    println!("final state: {}", board);
}
