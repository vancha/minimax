use std::cmp::{min, max};
use crate::Board;

pub fn minimax(node: Board, is_maximizing_player: bool) -> i32 {
    if node.is_terminal() {
        return node.heuristic_value();
    }

    if is_maximizing_player {
        let mut val = i32::MIN;
        for child in node.get_children(false) {
            val = max(val, minimax(child, false));
        }
        return val;
    } else {
        let mut val = i32::MAX;
        for child in node.get_children(true) {
            val = min(val, minimax(child, true));
        }
        return val;
    }
}

