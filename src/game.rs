// Main game logic, from game state to
// Main logic like research trees and others

use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::{data::game_data::GameData, input::handle_inputs, ui};

#[derive(Debug, Clone, Copy, Default)]
pub enum GameState {
    Closing,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InputMode {
    #[default]
    NormalMode,
    EditMode,
}

#[derive(Debug)]
pub struct Game {
    pub mode: InputMode,
    pub state: GameState,
    pub data: GameData,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

// Why the hell am i doing this again?
// Ohh yeah cause ::new looks better then ::default() :P
impl Game {
    pub fn new() -> Self {
        let terminal = ratatui::init();

        Self {
            mode: InputMode::default(),
            state: GameState::default(),
            data: GameData::new(),
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
