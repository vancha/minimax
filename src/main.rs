//use std::io::{stdin, stdout, Write};

pub mod board;
pub mod boardvalue;
pub mod minimax;
pub mod game;

use crate::board::*;
use crate::boardvalue::*;
use crate::minimax::*;
use crate::game::*;


fn main() {
    //create a game and play it :)
    let game = Game::new();
    game.play();
}
