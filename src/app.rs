use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::Frame;
use std::io;
use std::time::Duration;

use crate::solver::Solver;

pub struct App {
    pub inventory: Storage,
    pub capacity: usize,
    pub sack: Storage,
    exit: bool,
    solver: Box<dyn Solver>,
}

pub struct Storage {
    pub name: String,
    pub items: Vec<Item>,
}

impl Storage {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::default(),
        }
    }
    pub fn total(&self) -> usize {
        self.items.iter().map(|i| i.size).sum()
    }
    pub fn score(&self) -> f64 {
        self.items.iter().map(|i| i.weight).sum()
    }
}

#[derive(Default)]
pub struct Item {
    pub id: u64,
    pub size: usize,
    pub weight: f64,
    pub selected: bool,
}

impl App {
    pub fn new(solver: Box<dyn Solver>) -> Self {
        let mut inventory = Storage::new("Inventory");
        solver.setup(&mut inventory);
        Self {
            inventory,
            capacity: Default::default(),
            sack: Storage::new("Sack"),
            exit: Default::default(),
            solver,
        }
    }

    pub fn run(&mut self, terminal: &mut crate::tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(150))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
            }
        }
        self.clear_selected();
        self.solver
            .step(self.capacity, &mut self.inventory, &mut self.sack);
        Ok(())
    }

    fn clear_selected(&mut self) {
        self.inventory.items.iter_mut().for_each(|i| i.selected = false);
        self.sack.items.iter_mut().for_each(|i| i.selected = false);
    }
}
