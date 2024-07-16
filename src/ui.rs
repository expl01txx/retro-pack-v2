use super::model::{App, InputMode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Editing => (
            vec![
                Span::raw("[ RetroPack v2 ] Press "),
                Span::styled("Esc to", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" Exit, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - to Pack file"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);




    //filename
    let input = Paragraph::new(app.filename.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => {
                if app.row == 0 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Path"));
    f.render_widget(input, chunks[1]);

    //password
    let password = Paragraph::new(app.password.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => {
                if app.row == 1 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Password"));
    f.render_widget(password, chunks[2]);

    match app.input_mode {
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            if app.row == 0 {
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[1].x + app.filename.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                )
            } else {
                f.set_cursor(
                    chunks[2].x + app.password.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[2].y + 1,
                )
            }
        }
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Log"));
    f.render_widget(messages, chunks[3]);
}
