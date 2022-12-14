pub mod check_winner;

use std::fmt;

use crate::board::{BOARD_WIDTH, Board, board_slot::BoardSlot};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    One, Two,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            &Player::One => "Player one",
            &Player::Two => "Player two",
        };
        write!(f, "{}", s)
    }
}

pub enum Dir {
    Left, Right,
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
    cursor_pos: usize,
    board: Board,
}

impl Game {
    pub fn new(starting_player: Player) -> Self {
        Self {
            turn: starting_player,
            cursor_pos: 3,
            board: Board::new(),
        }
    }

    pub fn take_turn(&mut self) {
        let insertion_res = self.board.insert(self.turn.get_piece(), self.cursor_pos);
        if let Some(new_board) = insertion_res.ok() {
            self.board = new_board;
            self.turn = self.turn.other();
        }
    }

    pub fn move_cursor(&mut self, direction: Dir) {
        self.cursor_pos = match direction {
            Dir::Left => if self.cursor_pos != 0 { self.cursor_pos - 1 } else { BOARD_WIDTH - 1 },
            Dir::Right => if self.cursor_pos != BOARD_WIDTH - 1 { self.cursor_pos + 1 } else { 0 },
        };
    }

    #[cfg(test)]
    pub fn set_cursor(&mut self, location: usize) {
        self.cursor_pos = location % BOARD_WIDTH;
    }

    #[cfg(test)]
    pub fn get_board(&self) -> &Board {
        &self.board
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cursor = String::from(" ")
            + &"  ".repeat(self.cursor_pos)
            + "V-"
            + &self.turn.to_string();

        write!(f, "{}\r\n{}", cursor, self.board)
    }
}
