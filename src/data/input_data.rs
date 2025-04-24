use std::usize;

#[derive(Debug, Clone, Default)]
pub struct InputData {
    input_field: String,
    cursor_position: usize,
}

impl InputData {
    // handle content
    pub fn push_input(&mut self, input: char) {
        self.input_field.insert(self.cursor_position, input);
        self.move_cursor_right();
    }

    pub fn pop_last_input(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        // remove char on cursor position
        self.input_field.remove(self.cursor_position - 1);
        self.move_cursor_left();
    }

    pub fn get_string(&self) -> String {
        self.input_field.clone()
    }

    // handle cursor position
    pub fn move_cursor_left(&mut self) {
        let new_pos = self.cursor_position.saturating_sub(1);
        self.cursor_position = new_pos.clamp(0, self.input_field.len())
    }

    pub fn move_cursor_right(&mut self) {
        let new_pos = self.cursor_position.saturating_add(1);
        self.cursor_position = new_pos.clamp(0, self.input_field.len())
    }

    pub fn move_cursor_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }
}
