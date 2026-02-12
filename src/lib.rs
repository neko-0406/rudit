use ratatui::{DefaultTerminal, Frame, widgets::Widget};

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
    pub fn run (&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame));
            self.handle_events();
        }
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