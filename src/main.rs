// Nothing is suppost to happen here except connecting
// the render layer and the actuall game layer

use game::{Game, GameState};
use input::handle_inputs;
use render::Render;

pub mod render;
pub mod game;
pub mod input;
pub mod ui;
pub mod widgets;

fn main() {
    let mut game = Game::new();
    loop {
        Render::new(game).render();
        handle_inputs(&mut game);
        match game.state {
            GameState::Closing => {
                ratatui::restore();
                break;
            },
            GameState::Running => ()
        }
    }
}
