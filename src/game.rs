// Main game logic, from game state to
// Main logic like research trees and others

use std::io::stdout;

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Game {
     pub state: GameState 
}

impl Game {
    pub fn new() -> Self {
        Self { state: GameState::Running }
    }
}
