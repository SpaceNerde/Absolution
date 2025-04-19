// Main for rendering the games Graphics
// Including the UI, Maps and other stuff
// Will be split up in later iterations

use crate::game::Game;

#[derive(Debug)]
pub struct Render {
    game: Game
}

impl Render {
    pub fn new(game: Game) -> Self {
        Self { 
            game
        }
    }
}
