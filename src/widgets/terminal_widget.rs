// Widget to display terminal content

use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, text::{Line, Span, Text}, widgets::{Block, BorderType, List, ListDirection, ListItem, Paragraph, Widget}
};

use crate::data::terminal_data::TerminalData;

pub struct TerminalWidget {
    data: TerminalData,
}

impl TerminalWidget {
    pub fn new(terminal_data: TerminalData) -> Self {
        Self {
            data: terminal_data,
        }
    }
}

impl Widget for TerminalWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let interface = Block::bordered()
            .border_type(BorderType::Thick)
            .title("Terminal");

        interface.clone().render(area, buf);

        let interface_area = interface.inner(area);
        let interface_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
            ])
            .split(interface_area);
        
        let content: Vec<ListItem> = self.data.get_content()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Text::from(m.to_string());
                ListItem::new(content)
            })
            .rev()
            .collect();

        List::new(content)
            .direction(ListDirection::TopToBottom)
            .render(interface_layout[0], buf);
    }
}
