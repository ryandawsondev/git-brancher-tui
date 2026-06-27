#![allow(dead_code, unused_imports, unused_variables)]

use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod actions;
mod app;
mod input;
mod ops;
mod state;
mod ui;
mod update;
mod util;

use app::App;
use state::{data::MsgLevel, screen::Screen};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal).await;
    restore_terminal(&mut terminal)?;
    if let Err(ref e) = result {
        eprintln!("Error: {e:#}");
    }
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    if ops::config::gitrc_exists() {
        match ops::config::read_gitrc() {
            Ok(cfg) => app.config = Some(cfg),
            Err(e) => app.set_status(format!("Warning: {e}"), MsgLevel::Warn),
        }
    } else {
        app.screen = Screen::FirstRun(Default::default());
    }

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if let Some(task) = app.async_task.take() {
            if task.handle.is_finished() {
                match task.handle.await {
                    Ok(Ok(output)) => update::update(&mut app, actions::Action::TaskComplete(output)),
                    Ok(Err(e)) => app.set_status(format!("Error: {e:#}"), MsgLevel::Error),
                    Err(_) => app.set_status("Background task panicked".to_string(), MsgLevel::Error),
                }
            } else {
                app.async_task = Some(task);
            }
        }

        if crossterm::event::poll(Duration::from_millis(50))? {
            let ev = crossterm::event::read()?;
            if let Some(action) = input::handle_event(&app, ev) {
                update::update(&mut app, action);
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
