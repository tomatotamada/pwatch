use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::app::{App, AppMode};

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            match &app.mode {
                AppMode::Normal => handle_normal(app, key.code),
                AppMode::Search => handle_search(app, key.code),
                AppMode::Confirm { force } => {
                    let force = *force;
                    handle_confirm(app, key.code, force);
                }
            }
        }
    }
    Ok(())
}

fn handle_normal(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Char('d') => {
            if app.selected_port().is_some() {
                app.mode = AppMode::Confirm { force: false };
            }
        }
        KeyCode::Char('D') => {
            if app.selected_port().is_some() {
                app.mode = AppMode::Confirm { force: true };
            }
        }
        KeyCode::Char('/') => {
            app.filter.clear();
            app.selected = 0;
            app.mode = AppMode::Search;
        }
        KeyCode::Char('r') => app.refresh(),
        _ => {}
    }
}

fn handle_search(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter => {
            app.selected = 0;
            app.mode = AppMode::Normal;
        }
        KeyCode::Esc => {
            app.filter.clear();
            app.selected = 0;
            app.mode = AppMode::Normal;
        }
        KeyCode::Backspace => {
            app.filter.pop();
            app.selected = 0;
        }
        KeyCode::Char(c) => {
            app.filter.push(c);
            app.selected = 0;
        }
        _ => {}
    }
}

fn handle_confirm(app: &mut App, code: KeyCode, force: bool) {
    match code {
        KeyCode::Char('y') => app.kill_selected(force),
        KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Normal,
        _ => {}
    }
}
