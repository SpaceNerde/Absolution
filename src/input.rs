// Handle all inputs for the game
// Use state system in the game layer to connect to renderer

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::{command::handle_commands, game::Game};

pub fn handle_inputs(game: &mut Game) {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char(c) => game.data.push_char(c),
            KeyCode::Backspace => game.data.pop_char(),
            KeyCode::Left => game.data.move_cursor_left(),
            KeyCode::Right => game.data.move_cursor_right(),
            //handle_commands(game),
            KeyCode::Enter => {
                let input = game.data.get_string();
                game.command_registy.handle_commands(input)
            },
            _ => (),
        }
    }
}
