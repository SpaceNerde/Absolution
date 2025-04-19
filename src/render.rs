// Main for rendering the games Graphics
// Including the UI, Maps and other stuff
// Will be split up in later iterations

use std::io::{self, Stdout};

use ratatui::{crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}}, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::{game::Game, ui};

#[derive(Debug)]
pub struct Render {
    game: Game,
    terminal: Terminal<CrosstermBackend<Stdout>>
}

impl Render 
{
    pub fn new(game: Game) -> Self {
        // setup terminal
        // TODO: Add Error handling!
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        Self { 
            game,
            terminal
        }
    }

    pub fn render(mut self) {
        // TODO start on rendering
        drop(self.terminal.draw(ui::draw));
    }
}
