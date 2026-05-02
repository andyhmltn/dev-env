use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::KeyEvent;

use crate::flasher;
use crate::github::{self, RunInfo};
use crate::keys::{Action, KeyHandler, KeyMode};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Fetching,
    Downloading,
    WaitLeftHalf,
    FlashingLeft,
    WaitRightHalf,
    FlashingRight,
    Done,
    Error(String),
}

impl AppState {
    pub fn step_number(&self) -> usize {
        match self {
            Self::Fetching => 1,
            Self::Downloading => 2,
            Self::WaitLeftHalf => 3,
            Self::FlashingLeft => 4,
            Self::WaitRightHalf => 5,
            Self::FlashingRight => 6,
            Self::Done => 6,
            Self::Error(_) => 0,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Fetching => "Fetching",
            Self::Downloading => "Downloading",
            Self::WaitLeftHalf => "Left Half",
            Self::FlashingLeft => "Flashing Left",
            Self::WaitRightHalf => "Right Half",
            Self::FlashingRight => "Flashing Right",
            Self::Done => "Done",
            Self::Error(_) => "Error",
        }
    }
}

pub struct App {
    pub state: AppState,
    pub key_handler: KeyHandler,
    pub run_info: Option<RunInfo>,
    pub log: Vec<String>,
    pub log_offset: usize,
    pub left_uf2: Option<PathBuf>,
    pub right_uf2: Option<PathBuf>,
    pub left_flash_time: Option<String>,
    pub right_flash_time: Option<String>,
    pub should_quit: bool,
    pub spinner_tick: usize,
    flash_completed_at: Option<Instant>,
    fetch_rx: Option<Receiver<Result<RunInfo>>>,
    download_rx: Option<Receiver<Result<(PathBuf, PathBuf)>>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Fetching,
            key_handler: KeyHandler::new(),
            run_info: None,
            log: Vec::new(),
            log_offset: 0,
            left_uf2: None,
            right_uf2: None,
            left_flash_time: None,
            right_flash_time: None,
            should_quit: false,
            spinner_tick: 0,
            flash_completed_at: None,
            fetch_rx: None,
            download_rx: None,
        }
    }

    pub fn start_fetch(&mut self) {
        self.state = AppState::Fetching;
        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);
        thread::spawn(move || {
            let _ = tx.send(github::fetch_latest_run());
        });
    }

    fn start_download(&mut self) {
        self.state = AppState::Downloading;
        let run_id = self.run_info.as_ref().unwrap().id;
        let (tx, rx) = mpsc::channel();
        self.download_rx = Some(rx);
        thread::spawn(move || {
            let result = (|| -> Result<(PathBuf, PathBuf)> {
                let dir = github::download_artifacts(run_id)?;
                github::find_firmware_files(&dir)
            })();
            let _ = tx.send(result);
        });
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let action = self.key_handler.process(key);
        match action {
            Action::Quit => self.should_quit = true,
            Action::ScrollUp => {
                self.log_offset = self.log_offset.saturating_sub(1);
            }
            Action::ScrollDown => {
                if self.log_offset < self.log.len().saturating_sub(1) {
                    self.log_offset += 1;
                }
            }
            Action::ScrollTop => self.log_offset = 0,
            Action::ScrollBottom => {
                self.log_offset = self.log.len().saturating_sub(1);
            }
            Action::Skip => match self.state {
                AppState::WaitLeftHalf => self.state = AppState::WaitRightHalf,
                AppState::WaitRightHalf => self.state = AppState::Done,
                _ => {}
            },
            Action::Retry => {
                if matches!(self.state, AppState::Error(_)) {
                    if self.left_uf2.is_some() && self.right_uf2.is_some() {
                        if self.left_flash_time.is_none() {
                            self.state = AppState::WaitLeftHalf;
                        } else {
                            self.state = AppState::WaitRightHalf;
                        }
                    } else {
                        self.start_fetch();
                    }
                }
            }
            Action::Confirm | Action::None => {}
        }
    }

    pub fn tick(&mut self) {
        self.spinner_tick = self.spinner_tick.wrapping_add(1);

        match &self.state {
            AppState::Fetching => {
                if let Some(rx) = &self.fetch_rx {
                    if let Ok(result) = rx.try_recv() {
                        self.fetch_rx = None;
                        match result {
                            Ok(info) => {
                                self.log.push(format!("Build found: {}", info.title));
                                self.run_info = Some(info);
                                self.start_download();
                            }
                            Err(e) => self.state = AppState::Error(e.to_string()),
                        }
                    }
                }
            }
            AppState::Downloading => {
                if let Some(rx) = &self.download_rx {
                    if let Ok(result) = rx.try_recv() {
                        self.download_rx = None;
                        match result {
                            Ok((left, right)) => {
                                self.log.push("Firmware downloaded".into());
                                self.left_uf2 = Some(left);
                                self.right_uf2 = Some(right);
                                self.state = AppState::WaitLeftHalf;
                            }
                            Err(e) => self.state = AppState::Error(e.to_string()),
                        }
                    }
                }
            }
            AppState::WaitLeftHalf => {
                if let Some(volume) = flasher::check_volume() {
                    let uf2 = self.left_uf2.as_ref().unwrap();
                    match flasher::flash_firmware(uf2, &volume) {
                        Ok(()) => {
                            self.left_flash_time = Some(timestamp());
                            self.log.push("Left half flashed".into());
                            self.flash_completed_at = Some(Instant::now());
                            self.state = AppState::FlashingLeft;
                        }
                        Err(e) => self.state = AppState::Error(e.to_string()),
                    }
                }
            }
            AppState::FlashingLeft => {
                if let Some(t) = self.flash_completed_at {
                    if t.elapsed() >= Duration::from_secs(2) {
                        self.flash_completed_at = None;
                        self.log.push("Left half rebooted".into());
                        self.state = AppState::WaitRightHalf;
                    }
                }
            }
            AppState::WaitRightHalf => {
                if let Some(volume) = flasher::check_volume() {
                    let uf2 = self.right_uf2.as_ref().unwrap();
                    match flasher::flash_firmware(uf2, &volume) {
                        Ok(()) => {
                            self.right_flash_time = Some(timestamp());
                            self.log.push("Right half flashed".into());
                            self.flash_completed_at = Some(Instant::now());
                            self.state = AppState::FlashingRight;
                        }
                        Err(e) => self.state = AppState::Error(e.to_string()),
                    }
                }
            }
            AppState::FlashingRight => {
                if let Some(t) = self.flash_completed_at {
                    if t.elapsed() >= Duration::from_secs(2) {
                        self.flash_completed_at = None;
                        self.log.push("Right half rebooted".into());
                        self.state = AppState::Done;
                    }
                }
            }
            AppState::Done | AppState::Error(_) => {}
        }
    }

    pub fn command_buffer(&self) -> Option<&str> {
        match &self.key_handler.mode {
            KeyMode::Command(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

fn timestamp() -> String {
    let output = std::process::Command::new("date")
        .arg("+%H:%M:%S")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key_char(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn initial_state() {
        let app = App::new();
        assert_eq!(app.state, AppState::Fetching);
        assert!(!app.should_quit);
    }

    #[test]
    fn skip_left_half() {
        let mut app = App::new();
        app.state = AppState::WaitLeftHalf;
        app.handle_key(key_char('s'));
        assert_eq!(app.state, AppState::WaitRightHalf);
    }

    #[test]
    fn skip_right_half() {
        let mut app = App::new();
        app.state = AppState::WaitRightHalf;
        app.handle_key(key_char('s'));
        assert_eq!(app.state, AppState::Done);
    }

    #[test]
    fn quit_via_escape() {
        let mut app = App::new();
        app.handle_key(key(KeyCode::Esc));
        assert!(app.should_quit);
    }

    #[test]
    fn step_numbers() {
        assert_eq!(AppState::Fetching.step_number(), 1);
        assert_eq!(AppState::Downloading.step_number(), 2);
        assert_eq!(AppState::WaitLeftHalf.step_number(), 3);
        assert_eq!(AppState::FlashingLeft.step_number(), 4);
        assert_eq!(AppState::WaitRightHalf.step_number(), 5);
        assert_eq!(AppState::FlashingRight.step_number(), 6);
        assert_eq!(AppState::Done.step_number(), 6);
        assert_eq!(AppState::Error("test".into()).step_number(), 0);
    }

    #[test]
    fn retry_from_error_with_firmware_resumes_left() {
        let mut app = App::new();
        app.state = AppState::Error("flash failed".into());
        app.left_uf2 = Some(PathBuf::from("/tmp/left.uf2"));
        app.right_uf2 = Some(PathBuf::from("/tmp/right.uf2"));
        app.handle_key(key_char('r'));
        assert_eq!(app.state, AppState::WaitLeftHalf);
    }

    #[test]
    fn retry_from_error_with_firmware_resumes_right() {
        let mut app = App::new();
        app.state = AppState::Error("flash failed".into());
        app.left_uf2 = Some(PathBuf::from("/tmp/left.uf2"));
        app.right_uf2 = Some(PathBuf::from("/tmp/right.uf2"));
        app.left_flash_time = Some("12:00:00".into());
        app.handle_key(key_char('r'));
        assert_eq!(app.state, AppState::WaitRightHalf);
    }

    #[test]
    fn skip_does_nothing_during_fetch() {
        let mut app = App::new();
        app.state = AppState::Fetching;
        app.handle_key(key_char('s'));
        assert_eq!(app.state, AppState::Fetching);
    }
}
