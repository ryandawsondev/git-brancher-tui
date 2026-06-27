use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::{actions::Action, app::App, state::screen::Screen};

pub fn handle_event(app: &App, event: Event) -> Option<Action> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(app, key.code, key.modifiers),
        _ => None,
    }
}

fn handle_key(app: &App, code: KeyCode, modifiers: KeyModifiers) -> Option<Action> {
    // Global quit
    if matches!(code, KeyCode::Char('c')) && modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Action::Quit);
    }

    match &app.screen {
        Screen::ProfileCreate(form) | Screen::ProfileEdit { form, .. } | Screen::FirstRun(form) => {
            handle_form_key(code, modifiers)
        }
        _ => handle_nav_key(app, code),
    }
}

fn handle_form_key(code: KeyCode, modifiers: KeyModifiers) -> Option<Action> {
    match code {
        KeyCode::Esc => Some(Action::NavigateBack),
        KeyCode::Tab => Some(Action::TabNext),
        KeyCode::BackTab => Some(Action::TabPrev),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Backspace => Some(Action::InputBackspace),
        KeyCode::Char(c) => Some(Action::InputChar(c)),
        _ => None,
    }
}

fn handle_nav_key(app: &App, code: KeyCode) -> Option<Action> {
    match code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Esc => Some(Action::NavigateBack),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::SelectUp),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::SelectDown),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Char(' ') => Some(Action::SelectToggle),
        KeyCode::Char('r') => Some(Action::Refresh),
        KeyCode::Char('?') => Some(Action::HelpToggle),
        KeyCode::Tab => Some(Action::TabNext),
        KeyCode::BackTab => Some(Action::TabPrev),
        _ => None,
    }
}
