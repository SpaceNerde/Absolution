// Handle all inputs for the game
// Use state system in the game layer to connect to renderer

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::game::{Game, GameState, InputMode};

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
                KeyCode::Esc => {
                    game.state = GameState::Closing;
                }
                KeyCode::Char('n') => {
                    game.data.turn();
                }
                _ => ()
            },
            InputMode::EditMode => match key.code {
                KeyCode::Char(c) => game.data.push_char(c),
                KeyCode::Esc => game.mode = InputMode::NormalMode,
                KeyCode::Backspace => game.data.pop_char(),
                _ => ()
            }
        }
    }
}
