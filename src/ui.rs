// Simple Hardcoded UI thats how I love it!

use std::vec;

use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::Frame;

use crate::data::game_data::GameData;
use crate::systems::game_system::GameSystem;
use crate::widgets::campaign_widget::CampaignWidget;
use crate::widgets::input_widget::InputWidget;
use crate::widgets::resource_widget::ResourceWidget;
use crate::widgets::terminal_widget::TerminalWidget;


// TODO! Clean this up pls :,)
pub fn draw(frame: &mut Frame, data: GameData, system: GameSystem) {
    let outer_latout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(frame.area());
    
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(outer_latout[0]);

    let info_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(inner_layout[1]);

    frame.render_widget(ResourceWidget::new(data.clone()), info_layout[0]);
    frame.render_widget(
        TerminalWidget::new(data.get_terminal_data()),
        inner_layout[0],
    );

    frame.render_widget(InputWidget::new(data.get_input_data()), outer_latout[1]);

    // Set the Cursor Position to the actuall current position of the cursor
    frame.set_cursor_position(Position::new(
        outer_latout[1].x + data.get_cursor_position() as u16 + 1,
        outer_latout[1].y + 1,
    ));

    frame.render_widget(CampaignWidget::new(system.clone()), info_layout[1]);
}
