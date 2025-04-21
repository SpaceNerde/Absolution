use super::{input_data::InputData, terminal_data::TerminalData};

#[derive(Debug, Clone, Default)]
pub struct GameData {
    input_data: InputData,
    terminal_data: TerminalData,
    turns: i32,
    metals: f32,
    population: i32,
}

impl GameData {
    // input data handling
    pub fn get_input_data(&self) -> InputData {
        self.input_data.clone()
    }

    // terminal data handling
    pub fn get_terminal_data(&self) -> TerminalData {
        self.terminal_data.clone()
    }

    // turn handling
    pub fn get_turn(&self) -> i32 {
        self.turns
    }

    pub fn turn(&mut self) {
        self.turns += 1;
    }
    
    // population handling
    pub fn get_pop(&self) -> i32 {
        self.population
    }

    pub fn increase_pop(&mut self, pop: i32) {
        self.population += pop;
    }

    pub fn decrease_pop(&mut self, pop: i32) {
        self.population -= pop;
    }

    // metals handling
    pub fn get_metals(&self) -> f32 {
        self.metals
    }

    pub fn increase_metals(&mut self, metals: f32) {
        self.metals += metals;
    }

    pub fn decrease_metals(&mut self, metals: f32) {
        self.metals -= metals;
    }
}
