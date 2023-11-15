# minimax
a tic tac toe game using minimax in rust. Implements the algorithm as explained on wikipedia, with the following pseudo code:
```
function minimax(node, depth, maximizingPlayer) is
    if depth = 0 or node is a terminal node then
        return the heuristic value of node
    if maximizingPlayer then
        value := −∞
        for each child of node do
            value := max(value, minimax(child, depth − 1, FALSE))
        return value
    else (* minimizing player *)
        value := +∞
        for each child of node do
            value := min(value, minimax(child, depth − 1, TRUE))
        return value
```
The code for this can be found in src/minimax.rs:
```
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
}```
