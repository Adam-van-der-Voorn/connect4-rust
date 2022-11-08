use board::board_slot::BoardSlot;
use board::Board;
mod board;

fn main() {
    env_logger::init();

    let board = Board::new();
    let board = board.insert(BoardSlot::P1, 2).unwrap_or(board);
    let board = board.insert(BoardSlot::P2, 0).unwrap_or(board);
    let board = board.insert(BoardSlot::P1, 2).unwrap_or(board);
    let board = board.insert(BoardSlot::P2, 2).unwrap_or(board);
    let board = board.insert(BoardSlot::P1, 2).unwrap_or(board);

    println!("{}", board.to_string())
}
