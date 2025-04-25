// Handle all inputs for the game
// Use state system in the game layer to connect to renderer

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::{command::handle_commands, game::{Game, GameState, InputMode}};

pub fn handle_inputs(game: &mut Game) {
    if let Event::Key(key) = event::read().unwrap() {
        match game.mode {
            InputMode::NormalMode => match key.code {
                KeyCode::Char('e') => {
                    game.mode = InputMode::EditMode;
                }
                KeyCode::Char('q') => {
                    game.state = GameState::Closing;
                }
                KeyCode::Char('n') => {
                    game.data.turn();
                }
                KeyCode::Esc => {
                    game.state = GameState::Closing;
                }
                _ => (),
            },
            InputMode::EditMode => match key.code {
                KeyCode::Char(c) => game.data.push_char(c),
                KeyCode::Esc => game.mode = InputMode::NormalMode,
                KeyCode::Backspace => game.data.pop_char(),
                KeyCode::Left => game.data.move_cursor_left(),
                KeyCode::Right => game.data.move_cursor_right(),
                KeyCode::Enter => handle_commands(game),
                _ => (),
            },
        }
    }
}
