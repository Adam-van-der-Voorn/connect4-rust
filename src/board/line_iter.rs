use super::{board_slot::BoardSlot, Board};

pub enum BoardLineDir {
    Horizontal,
    Vertical,
    DiagonalDownhill,
    DiagonalUphill,
}

impl BoardLineDir {
    fn to_tuple(&self) -> (i32, i32) {
        match self {
            BoardLineDir::Horizontal => (1, 0),
            BoardLineDir::Vertical => (0, 1),
            BoardLineDir::DiagonalDownhill => (1, 1),
            BoardLineDir::DiagonalUphill => (1, -1),
        }
    }
}

pub struct BoardLineIter<'a> {
    board: &'a Board,
    dir: (i32, i32),
    loc: (i32, i32),
}

impl<'a> Iterator for BoardLineIter<'a> {
    type Item = &'a BoardSlot;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.board.get(self.loc.0, self.loc.1);
        self.loc = (self.loc.0 + self.dir.0, self.loc.1 + self.dir.1);
        return item;
    }
}

impl <'a> BoardLineIter<'a> {
    pub fn new(board: &'a Board, dir: BoardLineDir, origin: (i32, i32)) -> Option<BoardLineIter<'a>> {
        if board.get(origin.0, origin.1).is_none() {
            return None;
        }

        // walk back to find true origin
        let dir = dir.to_tuple();
        let mut true_origin: (i32, i32) = origin;

        loop {
            let delta: (i32, i32) = (true_origin.0 - dir.0, true_origin.1 - dir.1);
            if board.get(delta.0, delta.1).is_none() {
                break;
            }
            true_origin = delta;
        }

        Some(Self {
            board,
            dir,
            loc: true_origin,
        })
    }
}
