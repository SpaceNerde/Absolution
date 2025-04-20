// Widget to display terminal content

use ratatui::{buffer::Buffer, layout::{self, Constraint, Direction, Layout, Rect}, widgets::{Block, BorderType, Paragraph, Widget}};

use crate::data::terminal_data::TerminalData;

pub struct TerminalWidget {
    data: TerminalData
}

impl TerminalWidget {
    pub fn new(terminal_data: TerminalData) -> Self {
        Self {
            data: terminal_data 
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
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1)
            ])
            .split(interface_area);
        
        Paragraph::new("INFO: This is a test!").render(interface_layout[1], buf);
    }
}
