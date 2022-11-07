use board::Board;
use board::Piece;

mod board;

fn main() {
    env_logger::init();
    let board = Board::new();
    let board = board.insert(Piece::P1, 0);
    println!("{}", board.to_string())
}
    