use tui::{Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}};

use super::exec::CPU;
use std::io;

pub fn render_ui(cpu : &CPU) -> Result<(),io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //Make layout
    //20% Input Panel {
    //  80% Main UI (CPU,PC,ACC,IX,FLAGS)
    //}
    terminal.draw(|rec| {
            let main_layout = Layout::default()
                          .direction(Direction::Vertical)
                          .constraints([
                            Constraint::Percentage(20),
                            Constraint::Percentage(80),
                        ].as_ref())
                        .margin(1)
                        .split(rec.size());
            let cpu_layout = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Percentage(50),
                                Constraint::Percentage(50),
                            ].as_ref())
                            .split(main_layout[1]);
            let pc_and_flags = Layout::default();
                               
                                

    })?;
    
    Ok(())
}
