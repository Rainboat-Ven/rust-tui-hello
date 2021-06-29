mod menu;

use crate::menu::StatefulTable;
use std::{error::Error, io, io::Read, time::SystemTime};
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

//=============


//=============

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //缓冲区
    let mut buf = async_stdin();

    //菜单结构体初始化
    let mut menu_list = StatefulTable::new(vec![vec!["list1"], vec!["list2"], vec!["list3"]]);

    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let chunks0 = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(2), Constraint::Percentage(80)].as_ref())
                .split(f.size());

            let text = Text::from(format!("rust-TUI-hello\n{:?}", SystemTime::now()));
            f.render_widget(
                Paragraph::new(text).style(Style::default().bg(Color::White).fg(Color::Black)),
                chunks0[0],
            );

            let chunks1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks0[1]);

            let block = Block::default().title("菜单").borders(Borders::ALL);
            f.render_widget(block, chunks1[0]);

            let menu_rows = menu_list.items.iter().map(|l| {
                let height = l
                    .iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let cells = l.iter().map(|c| Cell::from(*c));
                Row::new(cells).height(height as u16).bottom_margin(1)
            });

            let t = Table::new(menu_rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Max(10),
                ]);
            f.render_stateful_widget(t, chunks1[0], &mut menu_list.state);

            let block = Block::default().borders(Borders::ALL);
            f.render_widget(block, chunks1[1]);
        })?;

        for b in buf.by_ref().keys() {
            match b.unwrap() {
                Key::Char('q') => {
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Down => {
                    menu_list.next();
                }
                Key::Up => {
                    menu_list.previous();
                }
                _ => (),
            }
        }
    }
}
