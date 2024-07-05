use crossterm::event::{self, Event, KeyCode};
use sim::Simulation;
use snake_tui::*;
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
    let mut sim = Simulation::new();
    loop {
        if event::poll(Duration::from_nanos(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        sim.update();
    }
    sim.terminate();
}
