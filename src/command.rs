// Command Handler to sanitize inputs and handle 
// the apropriate responce to those inputs

use crate::game::{Game, GameState};

const HELP_MESSAGE: &str = "
    Commands:\n
     exit - leave the game\n
     turn - go to the next turn\n
     help - display help message\n
";

// checks if the input from the input field matches with any commands
pub fn handle_commands(game: &mut Game) {
    // gets the current string inside the input field
    let prep_command = game.data.get_input_data().get_string();

    // match the given input with commands
    // TODO: Swap to nom for command parsing
    match prep_command.as_str() {
        "exit" => game.state = GameState::Closing,
        "turn" => game.data.turn(),
        "help" => game.data.push_content(HELP_MESSAGE.to_string()),
        _ => ()
    }
}
