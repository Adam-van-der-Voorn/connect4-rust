use std::io::{Stdout, Write};

use crossterm::{self, terminal, queue, cursor};

use crate::game::Game;

const DRAW_HEIGHT: u16 = 7;

pub fn init_draw(stdout: &mut Stdout, game: &mut Game) {
    write!(stdout, "{}", "\n".repeat(DRAW_HEIGHT.try_into().unwrap())).unwrap();
    draw(stdout, game)
}

pub fn draw(stdout: &mut Stdout, game: &mut Game) {
    queue!(stdout, cursor::MoveUp(DRAW_HEIGHT)).unwrap();
    queue!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    write!(stdout, "{}", &game).unwrap();
    if let Some(winner) = game.check_winner() {
        write!(stdout, "{} Wins!\r\n", winner).unwrap();
    }
    stdout.flush().unwrap();
}