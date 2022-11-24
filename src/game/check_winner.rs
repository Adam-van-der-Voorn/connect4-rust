use crate::board::{BOARD_HEIGHT_SIGN, BOARD_WIDTH_SIGN, line_iter::{BoardLineIter, BoardLineDir}};

use super::{Game, Player};

impl Game {
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
