use std::cell::RefCell;
use crate::{minimax, Board, BoardValue};
use std::io::{stdin, stdout, Write};


//game struct itself has only a board
pub struct Game {
    board:RefCell<Board>,
}

///asks the player for a new move using stdin, returns a grid index, convenience method
fn get_player_move() -> i32 {
    let mut user_input_buffer = String::new();
    print!("Please enter a number that corresponds to a cell");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut user_input_buffer)
        .expect("Not a valid input");

    user_input_buffer.trim().parse::<i32>().unwrap()
}

///takes a copy of the board, and returns a new board with it's best move, convenience
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

impl Game {
   pub fn new() -> Self {
        Game { board: RefCell::new(Board::new()) }
   }
   pub fn print_indices() {
        println!("\n_____________\n\n| 0 | 1 | 2 |\n_____________\n\n| 3 | 4 | 5 |\n_____________\n\n| 6 | 7 | 8 |\n_____________\n");
   }
   pub fn show_board(&self) {
        println!("{}",self.board.borrow());
   }
   pub fn play(&self) {
       while !&self.board.borrow().is_terminal() {
           let computer_move = get_computer_move(self.board.borrow().clone());
           *self.board.borrow_mut() = computer_move;

           if self.board.borrow().is_terminal() {
                break;
           }
           
           Game::print_indices();    
           println!("{}", self.board.borrow());

           let player_move = get_player_move() as usize;
           self.board.borrow_mut().grid[player_move] = BoardValue::O; 
       }
       println!("{}", self.board.borrow());
   }

}
