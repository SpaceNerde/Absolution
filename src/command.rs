// Command Handler to sanitize inputs and handle 
// the apropriate responce to those inputs

use crate::game::{Game, GameState};

// checks if the input from the input field matches with any commands
pub fn handle_commands(game: &mut Game) {
    // gets the current string inside the input field
    let prep_command = game.data.get_input_data().get_string();

    match prep_command.as_str() {
        "exit" => game.state = GameState::Closing,
        _ => ()
    }
}
