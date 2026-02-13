use std::io::stdout;

use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Frame, Terminal, prelude::CrosstermBackend, widgets::Widget};

#[derive(Debug)]
pub struct App {
    exit: bool
}

impl App {
    pub fn default() -> Self {
        Self { exit: false }
    }
}

impl App {
    pub fn run (&mut self) -> Result<(), std::io::Error> {
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        terminal.clear().unwrap();
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events();
        }
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
    
    pub fn draw(&self, frame: &mut Frame) {
        
    }
    
    pub fn handle_events(&mut self) {
        
    }
}

impl Widget for App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        
    }
}