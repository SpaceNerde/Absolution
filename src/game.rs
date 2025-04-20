// Main game logic, from game state to
// Main logic like research trees and others

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GameData {
    turns: i32,
    metals: f32,
    population: i32,
}

impl GameData {
    // turn handling
    pub fn get_turn(self) -> i32 {
        self.turns
    }

    pub fn turn(mut self) {
        self.turns += 1;
    }
    
    // population handling
    pub fn get_pop(self) -> i32 {
        self.population
    }

    pub fn increase_pop(mut self, pop: i32) {
        self.population += pop;
    }

    pub fn decrease_pop(mut self, pop: i32) {
        self.population -= pop;
    }

    // metals handling
    pub fn get_metals(self) -> f32 {
        self.metals
    }

    pub fn increase_metals(mut self, metals: f32) {
        self.metals += metals;
    }

    pub fn decrease_metals(mut self, metals: f32) {
        self.metals -= metals;
    }
}

#[derive(Debug, Clone, Copy, Default)]
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
