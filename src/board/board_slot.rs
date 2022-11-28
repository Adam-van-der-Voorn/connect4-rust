use std::fmt;
use strum_macros::IntoStaticStr;

use crate::game::Player;

#[derive(Copy, Clone, IntoStaticStr, PartialEq)]
pub enum BoardSlot {
    P1, P2, Empty
}

impl BoardSlot {
    pub fn to_player(&self) -> Option<Player> {
        match self {
            BoardSlot::P1 => Some(Player::One),
            BoardSlot::P2 => Some(Player::Two),
            _ => None
        }
    }
}

impl fmt::Display for BoardSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            &BoardSlot::P1 => '1',
            &BoardSlot::P2 => '2',
            &BoardSlot::Empty => ' '
        };
        write!(f, "{}", token)
    }
}

impl fmt::Debug for BoardSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self {
            &BoardSlot::P1 => '1',
            &BoardSlot::P2 => '2',
            &BoardSlot::Empty => '.'
        };
        write!(f, "{}", token)
    }
}