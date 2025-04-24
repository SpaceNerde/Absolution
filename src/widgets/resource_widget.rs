// Widget to display all current resources and the changes
// in resources in the next turn

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::data::game_data::GameData;

pub struct ResourceWidget {
    data: GameData,
}

impl ResourceWidget {
    pub fn new(game_data: GameData) -> Self {
        Self { data: game_data }
    }
}

impl Widget for ResourceWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let interface = Block::bordered()
            .border_type(BorderType::Thick)
            .title("Resources");

        interface.clone().render(area, buf);

        let interface_area = interface.inner(area);
        let interface_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(interface_area);

        let turn = format!("Turn: {}", self.data.get_turn());
        let population = format!("Population: {}", self.data.get_pop());
        let metals = format!("Metals: {}", self.data.get_metals());

        Paragraph::new(turn).render(interface_layout[0], buf);
        Paragraph::new(population).render(interface_layout[1], buf);
        Paragraph::new(metals).render(interface_layout[2], buf);
    }
}
