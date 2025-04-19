// Nothing is suppost to happen here except connecting
// the render layer and the actuall game layer

use game::Game;
use input::handle_inputs;
use render::Render;

pub mod render;
pub mod game;
pub mod input;

fn main() {
    let game = Game::new();
    loop {
        let render = Render::new(game);
        handle_inputs(game);
    }
}
