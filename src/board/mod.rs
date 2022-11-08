pub mod board_slot;

use std::fmt;
use board_slot::BoardSlot;
use log::debug;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

pub struct Board {
    grid: [[BoardSlot; BOARD_WIDTH]; BOARD_HEIGHT]
}

impl Board {
    pub fn insert(&self, piece: BoardSlot, slot: usize) -> Result<Board, &'static str> {
        debug!("Board insert for slot {}", slot);

        for row in (0..BOARD_HEIGHT).rev() {
            let cell = self.grid[row][slot];
            debug!("Board insert: considering cell {} at row={:?}.", cell, row);
            if cell == BoardSlot::Empty {
                let mut grid_clone = self.grid.clone();
                grid_clone[row][slot] = piece;
                return Ok(Board { grid: grid_clone })
            }
        }
        return Err("Column is full!");
    }

    pub fn new() -> Board {
        Board {
            grid: [[BoardSlot::Empty; BOARD_WIDTH]; BOARD_HEIGHT]
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