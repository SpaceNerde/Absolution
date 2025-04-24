use super::{input_data::InputData, resource_data::ResourceData, terminal_data::TerminalData};

#[derive(Debug, Clone, Default)]
pub struct GameData {
    input_data: InputData,
    terminal_data: TerminalData,
    turns: i32,
    resource_data: ResourceData
}

impl GameData {
    // input data handling
    pub fn get_input_data(&self) -> InputData {
        self.input_data.clone()
    }

    pub fn push_char(&mut self, input: char) {
        self.input_data.push_input(input);
    }

    pub fn pop_char(&mut self) {
        self.input_data.pop_last_input();
    }

    pub fn get_string(&self) -> String {
        self.input_data.get_string()
    }

    pub fn move_cursor_left(&mut self) {
        self.input_data.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.input_data.move_cursor_right();
    }

    pub fn move_cursor_start(&mut self) {
        self.input_data.move_cursor_start();
    }

    pub fn get_cursor_position(&self) -> usize {
        self.input_data.get_cursor_position()
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
        // give the player the resources on turn
        self.turns += 1;
    }

    // population handling
    pub fn get_pop(&self) -> i32 {
        self.resource_data.get_pop()
    }

    pub fn increase_pop(&mut self, pop: i32) {
        self.resource_data.increase_pop(pop);
    }

    pub fn decrease_pop(&mut self, pop: i32) {
        self.resource_data.decrease_pop(pop);
    }

    // metals handling
    pub fn get_metals(&self) -> f32 {
        self.resource_data.get_metals()
    }

    pub fn increase_metals(&mut self, metals: f32) {
        self.resource_data.increase_metals(metals);
    }

    pub fn decrease_metals(&mut self, metals: f32) {
        self.resource_data.decrease_metals(metals);
    }

    // mana handling
    pub fn get_mana(&self) -> f32 {
        self.resource_data.get_mana()
    }

    pub fn increase_mana(&mut self, mana: f32) {
        self.resource_data.increase_mana(mana);
    }

    pub fn decrease_mana(&mut self, mana: f32) {
        self.resource_data.decrease_mana(mana);
    }
}
