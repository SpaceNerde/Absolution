// Simple Hardcoded UI thats how I love it!

use std::vec;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{layout, Frame};

use crate::data::game_data::GameData;
use crate::widgets::input_widget::InputWidget;
use crate::widgets::terminal_widget::TerminalWidget;
use crate::widgets::resource_widget::ResourceWidget;

pub fn draw(frame: &mut Frame, data: GameData) {
    let outer_latout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(90),
            Constraint::Percentage(10)
        ])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(2, 3),
            Constraint::Ratio(1, 3)
        ])
        .split(outer_latout[0]);
    
    frame.render_widget(ResourceWidget::new(data.clone()), inner_layout[1]);
    frame.render_widget(TerminalWidget::new(data.get_terminal_data()), inner_layout[0]);

    frame.render_widget(InputWidget::new(data.get_input_data()), outer_latout[1]);
}
