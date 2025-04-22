// Simple Hardcoded UI thats how I love it!

use std::vec;

use ratatui::layout::{Constraint, Direction, Layout, Position};
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
    
    // Set the Cursor Position to the actuall current position of the cursor
    frame.set_cursor_position(Position::new(
        outer_latout[1].x + data.get_cursor_position() as u16 + 1,
        outer_latout[1].y + 1,
    ));
}
