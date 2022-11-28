mod board;
mod game;
mod test;
mod input;
mod draw;
mod raw_mode;

extern crate crossterm;

use crossterm::terminal::{self};
use crossterm::{execute};
use draw::{init_draw, draw};
use game::{Game, Player};
use std::io::{stdout};
use crate::input::{do_move, is_quit};

fn main() {
    let mut game = Game::new(Player::One);

    let _raw_mode_anchor = raw_mode::set_raw_mode();
    let mut stdout = stdout();
    let _ = execute!(stdout, terminal::SetTitle("connect4"));
    init_draw(&mut stdout, &mut game);
    let mut game_running = true;

    while game_running {
        if let Some(key_event) = crossterm::event::read().ok() {
            do_move(&key_event, &mut game);

            game_running = !is_quit(&key_event) &&
            game.check_winner().is_none();

            draw(&mut stdout, &mut game);
        }
    }    
}

