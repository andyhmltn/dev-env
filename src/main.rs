mod app;
mod banner;
mod git;
mod homebrew;
mod items;
mod keymap;
mod keys;
mod runner;
mod ui;

use std::io;
use std::panic;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{
    self, Event, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use ratatui::Terminal;

use app::App;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
    );
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let _ = execute!(terminal.backend_mut(), PopKeyboardEnhancementFlags);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn detect_repo_root() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("../../");
            if candidate.join("Cargo.toml").exists() {
                if let Ok(canonical) = candidate.canonicalize() {
                    return canonical;
                }
            }
        }
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() -> Result<()> {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    let repo_root = detect_repo_root();
    let mut terminal = setup_terminal()?;
    let mut app = App::new(repo_root.clone());

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if matches!(app.state, app::AppState::Main)
                    && matches!(
                        key.code,
                        crossterm::event::KeyCode::Enter
                    )
                    && app.launch_corne_flash()
                {
                    restore_terminal(&mut terminal)?;
                    let corne_dir = repo_root.join("keyboard/corne-flash");
                    let _ = std::process::Command::new("cargo")
                        .args(["run", "--release"])
                        .current_dir(&corne_dir)
                        .status();
                    terminal = setup_terminal()?;
                    continue;
                }
                app.handle_key(key);
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}
