// Main game logic, from game state to
// Main logic like research trees and others

use crate::data::game_data::GameData;

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub state: GameState,
    pub data: GameData,
}

impl Game {
    pub fn new() -> Self {
        Self { 
            state: GameState::Running,
            data: GameData::default()
        }
    }
}
