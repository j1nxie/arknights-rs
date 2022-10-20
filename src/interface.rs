use chrono::{DateTime, FixedOffset, Local, NaiveTime, Timelike, Utc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let utc: DateTime<Utc> = Utc::now();
    let now: DateTime<Local> = Local::now();
    let server_time = utc.with_timezone(&FixedOffset::west(7 * 3600));

    let time = utc.time();
    if (time.hour() == 11) || (time.minute() == 0) || (time.second() == 0) {}

    let now_block = Block::default().borders(Borders::ALL).title("Current time");
    let now_text = Paragraph::new(now.format("%r %B %d, %Y").to_string())
        .block(now_block)
        .alignment(Alignment::Center);
    f.render_widget(now_text, chunks[0]);

    let server_block = Block::default().borders(Borders::ALL).title("Server time");
    let server_text = Paragraph::new(server_time.format("%r %B %d, %Y").to_string())
        .block(server_block)
        .alignment(Alignment::Center);
    f.render_widget(server_text, chunks[1]);
}
