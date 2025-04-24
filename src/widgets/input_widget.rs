// Widget to display current input field


use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

use crate::data::input_data::{InputData};

pub struct InputWidget {
    data: InputData,
}

impl InputWidget {
    pub fn new(input_data_data: InputData) -> Self {
        Self {
            data: input_data_data,
        }
    }
}

impl Widget for InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let interface = Block::bordered()
            .border_type(BorderType::Thick)
            .title("Input");

        interface.clone().render(area, buf);

        let interface_area = interface.inner(area);
        let interface_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1)])
            .split(interface_area);

        let input_text = self.data.get_string();
        let input = Paragraph::new(input_text).wrap(Wrap { trim: true });
        input.render(interface_layout[0], buf);
    }
}
