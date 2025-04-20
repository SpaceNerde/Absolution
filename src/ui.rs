// Simple Hardcoded UI thats how I love it!

use std::vec;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{layout, Frame};

use crate::widgets::terminal_widget::TerminalWidget;
use crate::{widgets::resource_widget::ResourceWidget};
use crate::data::data::GameData;

pub fn draw(frame: &mut Frame, data: GameData) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(2, 3),
            Constraint::Ratio(1, 3)
        ])
        .split(frame.area());

    frame.render_widget(ResourceWidget::new(data.clone()), layout[1]);
    frame.render_widget(TerminalWidget::new(data.get_terminal_data().clone()), layout[0]);
}
