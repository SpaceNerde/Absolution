// Command Handler to sanitize inputs and handle 
// the apropriate responce to those inputs

use crate::{data::game_data::GameData, game::{Game, GameState}, systems::{campaigns::CampaignKind, game_system::GameSystem}};

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
        "turn" => {
            game.system.update(&mut game.data);
            game.data.turn()
        },
        "help" => game.data.push_content(HELP_MESSAGE.to_string()),
        "start campaign(mining)" => game.system.campaign.start_new(CampaignKind::MiningCampaign),
        _ => ()
    }
}

// all Commands inherite the command trait so and can be registered
// in the Command registry
//
// duhh...
pub trait Command {
    // not using a Vec cause i think clone would be more performance intensive... maybe
    fn matches(&self, input: &[&str]) -> bool;
    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem, state: &mut GameState);
}

// maybe i will impl a SubCommand trait at some point but not now i think 
// pub trait SubCommand {}

// Register Commands and match input with the whole registry to
// get the right command and execute it
#[derive(Default)]
pub struct CommandRegistry {
    commands: Vec<Box<dyn Command>>
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { 
            commands: vec![]
        }
    }

    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    
    // senitice and compare input to registerd commands and execute matches
    // idk i dont like to parse data, system and state all sepparte have to find something
    // better...., why am i even having the CommandRegistry inside the game struct?
    // TODO!
    pub fn handle_commands(&self, input: String, data: &mut GameData, system: &mut GameSystem, state: &mut GameState) {
        // TODO! handle invalid inputs
        let command_tokens: Vec<&str> = input.split_whitespace().collect::<_>();
        
        for cmd in &self.commands {
            if cmd.matches(&command_tokens) {
                cmd.execute(&command_tokens, data, system, state);
            }
        }
    }
}

pub struct TurnCommand;

impl Command for TurnCommand {
    fn matches(&self, input: &[&str]) -> bool {
        input.len() == 1 && input[0] == "turn"
    }

    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem, state: &mut GameState) {
        data.turn();
        system.update(data);

        data.push_content("Turn has passed.".to_string());
    }
}

pub struct ExitCommand;

impl Command for ExitCommand {
    fn matches(&self, input: &[&str]) -> bool {
        input.len() == 1 && input[0] == "exit"
    }

    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem, state: &mut GameState) {
        *state = GameState::Closing;
    }
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn matches(&self, input: &[&str]) -> bool {
        input.len() == 1 && input[0] == "help"
    }

    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem, state: &mut GameState) {
        data.push_content(HELP_MESSAGE.to_string());
    }
}

pub struct StartCommand;

impl Command for StartCommand {
    fn matches(&self, input: &[&str]) -> bool {
        input.len() == 3 && input[0] == "start"
    }

    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem, state: &mut GameState) {
        match args[1] {
            "campaign" => {
                match args[2] {
                    "mining" => {
                        system.start_new(CampaignKind::MiningCampaign);
                        data.push_content("Started Mining Campaign.".to_string());
                    },
                    "test" => (), // just here so nvim wont kill me :P TODO! remove asap
                    _ => ()
                }
            }
            // TODO! need to impl some kind of handling for those cases... meh later :P
            _ => () // no match for start sub command
        }
    }
}

/*
// simple command to test concept
pub struct TestCommand;

impl Command for TestCommand {
    // have to find a better way
    // maybe define this in a yml or something like this -> make Lua Viable
    fn matches(&self, input: &[&str]) -> bool {
        input.len() == 1 && input[0] == "test"
    }

    fn execute(&self, args: &[&str], data: &mut GameData, system: &mut GameSystem) {
    }

}
*/
