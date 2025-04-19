// Simple Hardcoded UI thats how I love it!

use ratatui::Frame;

pub fn draw(frame: &mut Frame) {
    frame.render_widget("Welcome to Absolution", frame.area());
}
