// Handle all inputs for the game
// Use state system in the game layer to connect to renderer

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::game::{Game, GameState};

pub fn handle_inputs(game: &mut Game) {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
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
        }
    }
}
