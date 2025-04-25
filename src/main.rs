// Nothing is suppost to happen here except connecting
// the render layer and the actuall game layer

use game::Game;

pub mod data;
pub mod game;
pub mod input;
pub mod ui;
pub mod widgets;
pub mod command;

fn main() {
    let mut game = Game::new();
    game.run();
}
