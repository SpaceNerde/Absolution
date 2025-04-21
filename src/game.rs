// Main game logic, from game state to
// Main logic like research trees and others

use crate::data::game_data::GameData;

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputMode {
    #[default]
    NormalMode,
    EditMode
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub mode: InputMode,
    pub state: GameState,
    pub data: GameData,
}


// Why the hell am i doing this again?
// Ohh yeah cause ::new looks better then ::default() :P
impl Game {
    pub fn new() -> Self {
        Self { 
            mode: InputMode::default(),
            state: GameState::default(),
            data: GameData::default()
        }
    }
}
