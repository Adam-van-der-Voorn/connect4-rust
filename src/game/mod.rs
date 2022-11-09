use super::BoardSlot;
use super::board::Board;

pub enum Player {
    One, Two
}

impl Player {
    pub fn get_piece(&self) -> BoardSlot {
        match self {
            &Player::One => BoardSlot::P1,
            &Player::Two => BoardSlot::P2
        }
    }

    pub fn other(&self) -> Player {
        match self {
            &Player::One => Player::Two,
            &Player::Two => Player::One
        }
    }
}

pub struct Game {
    turn: Player,
    board: Board,
    winner: Option<Player>
}

impl Game {
    pub fn new(starting_player: Player) -> Game {
        return Game { 
            turn: starting_player,
            board: Board::new(),
            winner: None
        }
    }

    pub fn take_turn(&mut self, slot: usize) -> Result<(), &str> {
        self.board = self.board.insert(self.turn.get_piece(), slot)?;
        self.turn = self.turn.other();
        return Ok(());
    }

    pub fn get_board(&self) -> &Board {
        return &self.board;
    }
}