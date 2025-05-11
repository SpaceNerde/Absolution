// Main game logic, from game state to
// Main logic like research trees and others

use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{command::{CommandRegistry, ExitCommand, HelpCommand, StartCommand, TurnCommand}, data::game_data::GameData, input::handle_inputs, systems::game_system::GameSystem, ui};

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

pub struct Game {
    pub state: GameState,
    pub data: GameData,
    pub system: GameSystem,
    pub command_registy: CommandRegistry,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Game {
    pub fn new() -> Self {
        let terminal = ratatui::init();
        
        let mut cmd_reg = CommandRegistry::new();

        cmd_reg.register(Box::new(ExitCommand));
        cmd_reg.register(Box::new(TurnCommand));
        cmd_reg.register(Box::new(HelpCommand));
        cmd_reg.register(Box::new(StartCommand));

        Self {
            state: GameState::default(),
            data: GameData::new(),
            system: GameSystem::new(),
            command_registy: cmd_reg,
            terminal,
        }
    }

    pub fn render(&mut self) {
        // TODO start on rendering
        drop(
            self.terminal
                .draw(|frame| ui::draw(frame, self.data.clone())),
        );
    }

    pub fn run(&mut self) {
        loop {
            self.render();
            handle_inputs(self);
            match self.state {
                GameState::Closing => {
                    ratatui::restore();
                    break;
                }
                GameState::Running => (),
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
