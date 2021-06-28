use std::{error::Error, io, io::Read};
use termion::{
    async_stdin, event::Key, input::MouseTerminal, input::TermRead, raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{
        BarChart, Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, TableState, Wrap,
    },
    Terminal,
};

use unicode_width::UnicodeWidthStr;

//=============
//commit try
//=============

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut buf = async_stdin();

    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let chunks0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());
            //
            let mut text = Text::from("标题");
            text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
            f.render_widget(Paragraph::new(text), chunks0[0]);
            //
            let chunks1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks0[1]);

            let block = Block::default().title("块2标题").borders(Borders::ALL);
            f.render_widget(block, chunks1[0]);

            let block = Block::default().title("块3标题").borders(Borders::ALL);
            f.render_widget(block, chunks1[1]);
        })?;

        for b in buf.by_ref().keys() {
            match b.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                _ => (),
            }
        }
    }
}
