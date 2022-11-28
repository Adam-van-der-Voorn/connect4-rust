use crossterm::event::{Event, KeyEvent, KeyCode, KeyModifiers};

use crate::game::{Dir, Game};

pub fn do_move(event: &Event, game: &mut Game) {
    match event {
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
        _ => (),
    }
}

pub fn is_quit(event: &Event) -> bool {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        }) => true,
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => true,
        _ => false,
    }
}