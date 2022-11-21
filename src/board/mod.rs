pub mod board_slot;
pub mod line_iter;

use std::{fmt, panic};
use board_slot::BoardSlot;
use log::debug;

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH_SIGN: i32 = 7;
pub const BOARD_HEIGHT_SIGN: i32 = 6;

pub struct Board {
    grid: [[BoardSlot; BOARD_WIDTH]; BOARD_HEIGHT]
}

impl Board {
    pub fn insert(&self, piece: BoardSlot, slot: usize) -> Result<Board, &'static str> {
        debug!("Board insert for slot {}", slot);
        if slot >= BOARD_WIDTH {
            panic!("Slot {} does not exist!", slot)
        }

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

    pub fn get(&self, x: i32, y: i32) -> Option<&BoardSlot> {
        if x < 0 || y < 0 || x >= BOARD_WIDTH_SIGN || y >= BOARD_HEIGHT_SIGN {
            return None;
        }
        let _x: usize = x.try_into().expect("x should be castable due to previous checks");
        let _y: usize = y.try_into().expect("y should be castable due to previous checks");
        self.grid.get(_y)
            .and_then(|row| row.get(_x))
    }

    pub fn get_grid(&self) -> &[[BoardSlot; BOARD_WIDTH]; BOARD_HEIGHT] {
        return &self.grid;
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
            s += "|";
            for cell in row {
                s += cell.to_string().as_str();
                s += "|"
            }
            s += "\n";
        }
        write!(f, "{}", s.as_str())
    }
}