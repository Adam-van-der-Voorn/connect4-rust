mod board;
mod game;
mod test;
mod util;

extern crate crossterm;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self};
use crossterm::{cursor, queue, execute};
use game::{Game, Player, Dir};
use std::io::{Write, stdout};

fn main() {
    let mut game = Game::new(Player::One);

    terminal::enable_raw_mode().expect("cannot enable raw mode");

    let mut stdout = stdout();
    let _ = execute!(stdout, terminal::SetTitle("connect4"));
    print!("{}", &game);

    loop {
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
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            _ => (),
        }
        queue!(stdout, cursor::MoveUp(7)).unwrap();
        queue!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        write!(stdout, "{}", &game).unwrap();
        stdout.flush().unwrap();
    }

    terminal::disable_raw_mode().expect("cannot disable raw mode");
}
