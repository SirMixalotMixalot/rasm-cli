/* use tui::{Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}};

use super::cpu::CPU;
use std::io;
 */
/* pub fn render_ui(_cpu : &CPU) -> Result<(),io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //Make layout
    //20% Input Panel {
    //  80% Main UI (CPU,PC,ACC,IX,FLAGS)
    //}
    //
    terminal.clear()?;    
    terminal.draw(|rec| {
            let main_layout = Layout::default()
                          .direction(Direction::Horizontal)
                          .constraints([
                            Constraint::Percentage(60),
                            Constraint::Percentage(40),
                            
                        ].as_ref())
                        .margin(1)
                        .split(rec.size());
            let cpu_block = Block::default().title("CPU").borders(Borders::all());
            rec.render_widget(cpu_block, main_layout[0]);
            let cpu_layout = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Percentage(50),
                                Constraint::Percentage(50),
                            ].as_ref())
                            .split(main_layout[0]);
            let pc_and_flags = Layout::default()
                               .direction(Direction::Horizontal)
                               .constraints([
                                Constraint::Length(1),
                                Constraint::Percentage(30),
                               ].as_ref())
                               .split(cpu_layout[0]);
            let acc_and_ix  = Layout::default()
                              .direction(Direction::Horizontal)
                              .constraints([
                                Constraint::Percentage(50),
                                Constraint::Percentage(50),
                              ].as_ref())
                              .split(cpu_layout[1]);

            let pc = Block::default().title("PC").borders(Borders::all());
            rec.render_widget(pc,pc_and_flags[1]);
            let flags = Block::default().title("Flags").borders(Borders::all());
            rec.render_widget(flags,pc_and_flags[0]);
            let acc = Block::default().title("ACC").borders(Borders::all());
            rec.render_widget(acc,acc_and_ix[0]);
            let ix = Block::default().title("IX").borders(Borders::all());
            rec.render_widget(ix, acc_and_ix[1]); 

    })?;
    
    Ok(())
}
 */