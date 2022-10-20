use crate::interface::ui;
use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

mod interface;

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        if poll(Duration::from_millis(500))? {
            if read()? == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }
        terminal.draw(ui)?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
