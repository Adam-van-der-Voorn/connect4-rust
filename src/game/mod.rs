use std::fmt;

use crate::board::{BOARD_WIDTH, BOARD_HEIGHT_SIGN, BOARD_WIDTH_SIGN, Board, board_slot::BoardSlot};

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
    winner: Option<Player>
}

impl Game {
    pub fn new(starting_player: Player) -> Self {
        Self {
            turn: starting_player,
            cursor_pos: 3,
            board: Board::new(),
            winner: None
        }
    }

    pub fn take_turn(&mut self) {
        self.board = self.board.insert(self.turn.get_piece(), self.cursor_pos)
            .expect("bad code, moves should be valid");
        self.turn = self.turn.other();
    }

    pub fn move_cursor(&mut self, direction: Dir) {
        self.cursor_pos = match direction {
            Dir::Left => if self.cursor_pos != 0 { self.cursor_pos - 1 } else { BOARD_WIDTH - 1 },
            Dir::Right => if self.cursor_pos != BOARD_WIDTH - 1 { self.cursor_pos + 1 } else { 0 },
        };
    }

    pub fn set_cursor(&mut self, location: usize) {
        self.cursor_pos = location % BOARD_WIDTH;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
    
    pub fn check_winner(&self) -> Option<Player> {
        // check horizontal
        for row in 0..BOARD_HEIGHT_SIGN {
            let potential_winner: Option<Player> = self.check_winner_for_line(0, row, 1, 0);
            if potential_winner.is_some() {
                return potential_winner;
            }
        }
 
        // check vertical
        for col in 0..BOARD_WIDTH_SIGN {
            let potential_winner: Option<Player> = self.check_winner_for_line(col, 0, 0, 1);
            if potential_winner.is_some() {
                return potential_winner;
            }
        }

        // check diagonal uphill
        // ...xxxx
        // ..xxxxx
        // .xxxxxx
        // xxxxxxx
        // xxxxxx.
        // xxxxx..
        // xxxx...
        for col in -3..BOARD_WIDTH_SIGN - 3 {
            let bottom_row = BOARD_HEIGHT_SIGN - 1;
            let amount_out_of_bounds = if col < 0 { col.abs() } else { 0 };
            let row = bottom_row - amount_out_of_bounds;
            let adjusted_col = if col >= 0 { col } else { 0 };
            let potential_winner: Option<Player> = self.check_winner_for_line(adjusted_col, row, 1, -1);
            if potential_winner.is_some() {
                return potential_winner;
            }
        }

        // check diagonal downhill
        // xxxx...
        // xxxxx..
        // xxxxxx.
        // xxxxxxx
        // .xxxxxx
        // ..xxxxx
        // ...xxxx
        for col in -3..BOARD_WIDTH_SIGN - 3 {
            let amount_out_of_bounds = if col < 0 { col.abs() } else { 0 };
            let row = amount_out_of_bounds;
            let adjusted_col = if col >= 0 { col } else { 0 };
            let potential_winner: Option<Player> = self.check_winner_for_line(adjusted_col, row, 1, 1);
            if potential_winner.is_some() {
                return potential_winner;
            }
        }
        return None;
    }

    pub fn check_winner_for_line(&self, x_start: i32, y_start: i32, x_dir: i32, y_dir: i32) -> Option<Player> {
        let mut x = x_start;
        let mut y = y_start;
        let mut run = 0;
        let mut winner_candidate: Option<Player> = None;

        let mut _cell: Option<&BoardSlot> = self.board.get(x, y);

        while _cell.is_some() {
            let cell = *_cell.unwrap();
            // init run
            if winner_candidate.is_none() && cell != BoardSlot::Empty {
                winner_candidate = Some(cell.to_player().expect("boardslot should not be empty"));
                run += 1;
            }
            else if winner_candidate.is_some() {
                // continue run
                if cell.to_player().is_some() && winner_candidate.unwrap() == cell.to_player().unwrap() {
                    run += 1;
                }
                // end/restart run
                else {
                    winner_candidate = cell.to_player();
                    run = if winner_candidate.is_some() { 1 } else { 0 };
                }
            }

            if run == 4 {
                assert!(winner_candidate.is_some(), "run is 4 but no winner is stored");
                return winner_candidate;
            }

            x += x_dir;
            y += y_dir;
            _cell = self.board.get(x, y);
        }
        return None;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cursor = String::from(" ")
            + &"  ".repeat(self.cursor_pos)
            + "V-"
            + &self.turn.to_string();

        write!(f, "{}\n{}", cursor, self.board)
    }
}
