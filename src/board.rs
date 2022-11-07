use std::fmt;
use log::error;
use strum_macros::IntoStaticStr;

use crate::board::board_slot::BoardSlot;

pub mod board_slot;

#[derive(Copy, Clone, IntoStaticStr)]
pub enum Piece {
    P1, P2
}

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

pub struct Board {
    grid: [[BoardSlot; BOARD_HEIGHT]; BOARD_WIDTH]
}

impl Board {
    pub fn insert(&self, piece: Piece, _slot: i32) -> Board {
        let s2: &'static str = piece.into();
        error!("How can I insert a peice into {} if I am unimplemented??", s2);
        return Board::new();
    }

    pub fn new() -> Board {
        Board {
            grid: [[BoardSlot::Empty; BOARD_HEIGHT]; BOARD_WIDTH]
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for row in self.grid {
            for cell in row {
                s += cell.to_string().as_str();
                s += "|"
            }
            s += "\n";
        }
        write!(f, "{}", s.as_str())
    }
}