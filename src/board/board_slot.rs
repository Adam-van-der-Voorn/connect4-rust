use std::fmt;
use strum_macros::IntoStaticStr;

#[derive(Copy, Clone, IntoStaticStr, PartialEq, Debug)]
pub enum BoardSlot {
    P1, P2, Empty
}

impl fmt::Display for BoardSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token: char;
        match self {
            &BoardSlot::P1 => token = '1',
            &BoardSlot::P2 => token = '2',
            &BoardSlot::Empty => token = ' '
        }
        write!(f, "{}", token)
    }
}