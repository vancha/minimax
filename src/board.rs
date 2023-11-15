use crate::BoardValue;
use std::fmt;


//the board, has a total of 8 values, cause there are 8 cells
#[derive(Clone, Debug)]
pub struct Board {
    pub grid: Vec<BoardValue>,
}

//make sure the board itself can be printed to the screen
impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "\n_____________\n\n|{:?}|{:?}|{:?}|\n_____________\n\n|{:?}|{:?}|{:?}|\n_____________\n\n|{:?}|{:?}|{:?}|\n_____________\n",
            self.grid[0],
            self.grid[1],
            self.grid[2],
            self.grid[3],
            self.grid[4],
            self.grid[5],
            self.grid[6],
            self.grid[7],
            self.grid[8],
            )
    }
}

impl Board {
    //if someone has won, or there's no more empty space on the board, this board is at a terminal
    //state
    pub fn is_terminal(&self) -> bool {
        if self.maximizing_player_won() || self.minimizing_player_won() || self.board_full() {
            return true;
        }
        return false;
    }

    //if not a single space on the board is empty, it's full 
    pub fn board_full(&self) -> bool {
        for value in &self.grid {
            if value == &BoardValue::EMPTY {
                return false
            }
        }
        return true;
    }
    //if an entire row has the value of val
    pub fn any_row_has_val(&self, val: BoardValue) -> bool {
        //checks the top row, index 0,1 and 2
        if self.grid.get(0) == self.grid.get(1)
            && self.grid.get(1) == self.grid.get(2)
            && self.grid.get(0).unwrap() == &val
        {
            return true;
        }
        //middle row, index 3,4 and 5 
        if self.grid.get(3) == self.grid.get(4)
            && self.grid.get(4) == self.grid.get(5)
            && self.grid.get(3).unwrap() == &val
        {
            return true;
        }
        //bottom row, index 6,7 and 8
        if self.grid.get(6) == self.grid.get(7)
            && self.grid.get(7) == self.grid.get(8)
            && self.grid.get(6).unwrap() == &val
        {
            return true;
        }

        //no entire rows have the value of val
        return false;
    }
    //if an entire column has the value of val
    pub fn any_column_has_val(&self, val: BoardValue) -> bool {
        //left column, index 0, 3 and 6
        if self.grid.get(0) == self.grid.get(3)
            && self.grid.get(3) == self.grid.get(6)
            && self.grid.get(0).unwrap() == &val
        {
            return true;
        }
        //middle column, index 1, 4 and 7
        if self.grid.get(1) == self.grid.get(4)
            && self.grid.get(4) == self.grid.get(7)
            && self.grid.get(1).unwrap() == &val
        {
            return true;
        }
        //right column, index 2, 5 and 8
        if self.grid.get(2) == self.grid.get(5)
            && self.grid.get(5) == self.grid.get(8)
            && self.grid.get(2).unwrap() == &val
        {
            return true;
        }
        return false;
    }
    //if one of either entire diagonals has the value of val
    pub fn any_diagonal_has_val(&self, val: BoardValue) -> bool {
        //top left to bottom right, index 0, 4 and 8
        if self.grid.get(0) == self.grid.get(4)
            && self.grid.get(4) == self.grid.get(8)
            && self.grid.get(0).unwrap() == &val {
            return true;
        }
        //top right to bottom left diagonal, indx 2, 4 and 6
        if self.grid.get(2) == self.grid.get(4)
                && self.grid.get(4) == self.grid.get(6)
                && self.grid.get(2).unwrap() == &val {
            return true;
        }
        //there are no entire diagonals with the value of val
        return false;
    }
    
    //the player has one if there's only X's either in an entire column, row, or diagonal
    pub fn maximizing_player_won(&self) -> bool {
        if self.any_column_has_val(BoardValue::X) {
            return true;
        }
        if self.any_row_has_val(BoardValue::X) {
            return true;
        }
        if self.any_diagonal_has_val(BoardValue::X) {
            return true;
        }
        return false;
    }

    pub fn minimizing_player_won(&self) -> bool {
        if self.any_column_has_val(BoardValue::O) {
            return true;
        }
        if self.any_row_has_val(BoardValue::O) {
            return true;
        }
        if self.any_diagonal_has_val(BoardValue::O) {
            return true;
        } 
        return false;
    }

    pub fn get_children(&self, maximizing: bool) -> Vec<Self> {
        let mut children = vec![];

        for (index, value) in self.grid.iter().enumerate() {
            if value == &BoardValue::EMPTY {
                //make a new board out of the current one
                let mut cloned_board = self.clone();
                //change the state
                cloned_board.grid[index] = match maximizing {
                    true => BoardValue::X,
                    false => BoardValue::O,
                };
                children.push(cloned_board);
            }
        }
        children
    }
    
    //evaluate the board, and return an heuristic value
    // 10 if maximizing player wins, -10 if minimizing player wins, 0 if board is full
    pub fn heuristic_value(&self) -> i32 {
        if !self.is_terminal() {
            0
        } else {
            if self.maximizing_player_won() {
                return 1;
            }
            else if self.minimizing_player_won() {
                return -1;
            }
            else {//if self.board_full() {
                return 0;
            }
        }
    }


    pub fn new() -> Self {
        Board {
            grid: vec![
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
                BoardValue::EMPTY,
            ],
        }
    }
}
