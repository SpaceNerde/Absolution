#[derive(Debug, Clone, Default)]
pub struct TerminalData {
    content: Vec<String>
}

impl TerminalData {
    // handle content
    pub fn push_content(mut self, content: String) {
        self.content.push(content);
    }

    pub fn pop_last_content(mut self) {
        self.content.remove(self.content.len() - 1);
    }
}
