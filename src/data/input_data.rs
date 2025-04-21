#[derive(Debug, Clone, Default)]
pub struct InputData {
    input_field: Vec<char>
}

impl InputData {
    // handle content
    pub fn push_input(mut self, input: char) {
        self.input_field.push(input);
    }

    pub fn pop_last_input(mut self) {
        self.input_field.remove(self.input_field.len() - 1);
    }
}
