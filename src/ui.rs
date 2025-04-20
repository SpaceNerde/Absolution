// Simple Hardcoded UI thats how I love it!

use ratatui::Frame;

use crate::{game::GameData, widgets::resource_widget::ResourceWidget};

pub fn draw(frame: &mut Frame, data: &GameData) {
    frame.render_widget(ResourceWidget::new(*data), frame.area());
}
