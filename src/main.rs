// Nothing is suppost to happen here except connecting
// the render layer and the actuall game layer

use game::{Game, GameState};
use input::handle_inputs;

pub mod game;
pub mod input;
pub mod ui;
pub mod widgets;
pub mod data;

fn main() {
    let mut game = Game::new();
    game.run();
}
