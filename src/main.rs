mod board;
mod game;
mod test;
mod util;
mod input;

extern crate crossterm;

use crossterm::terminal::{self};
use crossterm::{cursor, queue, execute};
use game::{Game, Player};
use std::io::{Write, stdout};

use crate::input::{do_move, is_quit};

fn main() {
    let mut game = Game::new(Player::One);

    terminal::enable_raw_mode().expect("should be able to enable raw mode, this is required to draw to the screen");

    let mut stdout = stdout();
    let _ = execute!(stdout, terminal::SetTitle("connect4"));
    print!("{}", &game);

    loop {
        if let Some(key_event) = crossterm::event::read().ok() {
            do_move(&key_event, &mut game);

            if is_quit(&key_event) {
                break;
            }
        }
        
        queue!(stdout, cursor::MoveUp(7)).unwrap();
        queue!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        write!(stdout, "{}", &game).unwrap();
        stdout.flush().unwrap();

        if let Some(winner) = game.check_winner() {
            println!("{} wins!", winner);
            break;
        }
    }

    terminal::disable_raw_mode().expect("should be able to disable raw mode");
}
