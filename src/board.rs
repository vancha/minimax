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
    ///if someone has won, or the board is full, then were at a terminal state
    pub fn is_terminal(&self) -> bool {
        self.maximizing_player_won() || self.minimizing_player_won() || self.board_full()
    }

    ///if not a single space on the board is empty, it's full
    pub fn board_full(&self) -> bool {
        !&self.grid.iter().any(|item| item == &BoardValue::EMPTY)
    }

    ///checks if any entire row is filled with val
    pub fn any_row_has_val(&self, val: BoardValue) -> bool {
        //top row, indices 0 1 and 2
        self.grid.get(0) == self.grid.get(1) && self.grid.get(1) == self.grid.get(2) && self.grid.get(0).unwrap() == &val ||
            //middle row, indices 3, 4 and 5
            self.grid.get(3) == self.grid.get(4) && self.grid.get(4) == self.grid.get(5) && self.grid.get(3).unwrap() == &val ||
            //bottom row, indices 6, 7 and 8
            self.grid.get(6) == self.grid.get(7) && self.grid.get(7) == self.grid.get(8) && self.grid.get(6).unwrap() == &val
    }
    ///if an entire column has the value of val
    pub fn any_column_has_val(&self, val: BoardValue) -> bool {
        //left column, index 0, 3 and 6
        self.grid.get(0) == self.grid.get(3) && self.grid.get(3) == self.grid.get(6) && self.grid.get(0).unwrap() == &val ||
            //middle column
            self.grid.get(1) == self.grid.get(4) && self.grid.get(4) == self.grid.get(7) && self.grid.get(1).unwrap() == &val ||
            //rightmost column
            self.grid.get(2) == self.grid.get(5) && self.grid.get(5) == self.grid.get(8) && self.grid.get(2).unwrap() == &val
    }
    ///if one of either entire diagonals has the value of val
    pub fn any_diagonal_has_val(&self, val: BoardValue) -> bool {
        //top left to bottom right, index 0, 4 and 8
        self.grid.get(0) == self.grid.get(4) && self.grid.get(4) == self.grid.get(8) && self.grid.get(0).unwrap() == &val ||
            //top right to bottom left diagonal
            self.grid.get(2) == self.grid.get(4) && self.grid.get(4) == self.grid.get(6) && self.grid.get(2).unwrap() == &val
    }

    ///the maximizing player has won if there's only X's either in an entire column, row, or diagonal
    pub fn maximizing_player_won(&self) -> bool {
        self.any_column_has_val(BoardValue::X) || self.any_row_has_val(BoardValue::X) || self.any_diagonal_has_val(BoardValue::X)
    }

    ///same as mazimizing player, but with O's
    pub fn minimizing_player_won(&self) -> bool {
        self.any_column_has_val(BoardValue::O) || self.any_row_has_val(BoardValue::O) || self.any_diagonal_has_val(BoardValue::O)
    }

    ///generates all possible moves, or "children" for the maximizing or not-mazimizing player
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

    ///evaluate the board, and return the heuristic value
    pub fn heuristic_value(&self) -> i32 {
        if !self.is_terminal() {
            0
        } else {
            if self.maximizing_player_won() {
                return 1;
            } else if self.minimizing_player_won() {
                return -1;
            } else {
                return 0;
            }
        }
    }

    pub fn new() -> Self {
        Board {
            grid: vec![
                BoardValue::EMPTY, //0,0
                BoardValue::EMPTY, //1,0
                BoardValue::EMPTY, //2,0
                BoardValue::EMPTY, //0,1
                BoardValue::EMPTY, //1,1
                BoardValue::EMPTY, //2,1
                BoardValue::EMPTY, //0,2
                BoardValue::EMPTY, //1,2
                BoardValue::EMPTY, //2,2
            ],
        }
    }
}
