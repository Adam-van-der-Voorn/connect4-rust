mod board;
mod game;
mod test;
mod util;

extern crate crossterm;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self};
use game::{Game, Player, Dir};

use crate::util::cls_string;

fn main() {
    let mut game = Game::new(Player::One);

    terminal::enable_raw_mode().expect("cannot enable raw mode");

    loop {
        print!("{}{}", cls_string(), &game);
        match crossterm::event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) => game.move_cursor(Dir::Left),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => game.move_cursor(Dir::Right),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => game.take_turn(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) => break,
            _ => (),
        }
    }

    terminal::disable_raw_mode().expect("cannot disable raw mode");
}
