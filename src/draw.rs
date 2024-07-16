use crate::pack::process;

use super::model::{App, InputMode};
use super::ui::ui;

use std::io;

use crossterm::event::{self, Event, KeyCode};

use tui::{backend::Backend, Terminal};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if app.row < 1 {
                            app.row += 1;
                        } else {
                            app.messages.push("Starting...".to_string());
                            app.messages.push(process(
                                app.filename.to_string(),
                                app.password.to_string(),
                            ));
                        }
                    }
                    KeyCode::Char(c) => {
                        if app.row == 0 {
                            app.filename.push(c);
                        } else {
                            app.password.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.row == 0 {
                            app.filename.pop();
                        } else {
                            app.password.pop();
                        }
                    }
                    KeyCode::Up => {
                        if app.row != 0 {
                            app.row -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.row < 1 {
                            app.row += 1;
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
            }
        }
    }
}
