use std::fmt;

use crate::board::{BOARD_WIDTH, BOARD_HEIGHT_SIGN, BOARD_WIDTH_SIGN, Board, board_slot::BoardSlot, line_iter::{BoardLineIter, BoardLineDir}};

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
        let mut line_iters: Vec<BoardLineIter> = vec![];

        // check horizontal
        for row in 0..BOARD_HEIGHT_SIGN {
            let it = BoardLineIter::new(&self.board, BoardLineDir::Horizontal, (0, row))
            .expect("for loop should be in bounds");
            line_iters.push(it);
        }
 
        // check vertical
        for col in 0..BOARD_WIDTH_SIGN {
            let it = BoardLineIter::new(&self.board, BoardLineDir::Vertical, (col, 0))
            .expect("for loop should be in bounds");
            line_iters.push(it);
        }

        // check diagonal downhill and diagonal uphill
        // xxxx...    ...xxxx
        // xxxxx..    ..xxxxx
        // xxxxxx.    .xxxxxx
        // xxxxxxx    xxxxxxx
        // .xxxxxx    xxxxxx.
        // ..xxxxx    xxxxx..
        // ...xxxx    xxxx...
        for col in -3..BOARD_WIDTH_SIGN - 3 {
            let amount_out_of_bounds = if col < 0 { col.abs() } else { 0 };
            let col = if col >= 0 { col } else { 0 };
            { // diagonal downhill
                let row = amount_out_of_bounds;
                let it = BoardLineIter::new(&self.board, BoardLineDir::DiagonalDownhill, (col, row))
                .expect("for loop should be in bounds");
                line_iters.push(it);
            }
            { // diagonal uphill
                let bottom_row = BOARD_HEIGHT_SIGN - 1;
                let row = bottom_row - amount_out_of_bounds;
                let it = BoardLineIter::new(&self.board, BoardLineDir::DiagonalUphill, (col, row))
                .expect("for loop should be in bounds");
                line_iters.push(it);
            }    
        }

        for mut it in line_iters {
            if let Some(winner) = self.check_winner_for_line(&mut it) {
                return Some(winner);
            }
        }

        return None;
    }

    pub fn check_winner_for_line(&self, it: &mut BoardLineIter<'_>) -> Option<Player> {
        struct Run {
            count: i32,
            winnder_candidate: Option<Player>
        }

        impl Run {
            fn start(player: Player) -> Self {
                return Self { count: 1, winnder_candidate: Some(player) }
            }

            fn advance(&self) -> Self {
                return Self { count: self.count + 1, ..*self };
            }

            fn new() -> Self {
                Self { count: 0, winnder_candidate: None }
            }

            fn get_winner(&self) -> Option<Player> {
                if self.count >= 4 {
                    return Some(self.winnder_candidate.expect("run is > 0, so a winner should be stored"));
                }
                return None;
            }
        }

        let mut run = Run::new();

        loop {
            if let Some(cell) = it.next() {
                match cell.to_player() {
                    Some(player_on_cell) => {
                        if let Some(candidate) = run.winnder_candidate {
                            if candidate == player_on_cell {
                                run = run.advance();
                            }
                            else {
                                run = Run::start(player_on_cell);
                            }
                        }
                        else {
                            run = Run::start(player_on_cell);
                        }
                    },
                    None => run = Run::new()
                };
            }
            else {
                return None;
            }

            if let Some(winner) = run.get_winner() {
                return Some(winner);
            }
        }
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
