use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crossterm::event::KeyEvent;

use crate::git;
use crate::homebrew::{self, UntrackedPkg};
use crate::keymap;
use crate::items::{self, ItemId, ItemKind, MenuItem, SyncStatus};
use crate::keys::{Action, KeyHandler, KeyMode};
use crate::runner::{self, RunnerMsg};

#[derive(Debug)]
pub enum BrewSyncState {
    Loading,
    Prompting(usize),
    CommentInput(usize),
    Done(usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GitBanner {
    Checking,
    Behind(usize),
    Pulling,
    UpToDate,
    Failed,
}

pub enum AppState {
    Main,
    Running(usize),
    HomebrewSync(BrewSyncState),
    KeyboardLayout(usize),
    Error(String),
}

pub struct App {
    pub state: AppState,
    pub key_handler: KeyHandler,
    pub items: Vec<MenuItem>,
    pub selected: usize,
    pub should_quit: bool,
    pub spinner_tick: usize,
    pub repo_root: PathBuf,
    pub command_output: Vec<String>,
    pub scroll_offset: usize,
    pub brew_untracked: Vec<UntrackedPkg>,
    pub brew_added: usize,
    pub brew_skipped: usize,
    pub brew_comment: Option<String>,
    pub cached_keymap: Option<keymap::Keymap>,
    pub highlight_ticks: [u8; 42],
    pub git_banner: GitBanner,
    pub search_query: Option<String>,
    git_check_rx: Option<mpsc::Receiver<GitCheckResult>>,
    git_pull_rx: Option<mpsc::Receiver<anyhow::Result<String>>>,
    status_receivers: Vec<(usize, mpsc::Receiver<SyncStatus>)>,
    runner_rx: Option<mpsc::Receiver<RunnerMsg>>,
    brew_sync_rx: Option<mpsc::Receiver<anyhow::Result<Vec<UntrackedPkg>>>>,
}

enum GitCheckResult {
    UpToDate,
    Behind(usize),
    Failed,
}

impl App {
    pub fn new(repo_root: PathBuf) -> Self {
        let mut app = Self {
            state: AppState::Main,
            key_handler: KeyHandler::new(),
            items: MenuItem::all(),
            selected: 0,
            should_quit: false,
            spinner_tick: 0,
            repo_root,
            command_output: Vec::new(),
            scroll_offset: 0,
            brew_untracked: Vec::new(),
            brew_added: 0,
            brew_skipped: 0,
            brew_comment: None,
            git_check_rx: None,
            git_pull_rx: None,
            status_receivers: Vec::new(),
            runner_rx: None,
            brew_sync_rx: None,
            cached_keymap: None,
            highlight_ticks: [0; 42],
            git_banner: GitBanner::Checking,
            search_query: None,
        };
        app.start_git_check();
        app.start_status_checks();
        app
    }

    pub fn start_git_check(&mut self) {
        let (tx, rx) = mpsc::channel();
        let repo_root = self.repo_root.clone();
        self.git_check_rx = Some(rx);

        thread::spawn(move || {
            let result = match git::check_remote(&repo_root) {
                Ok(git::GitStatus::UpToDate) => GitCheckResult::UpToDate,
                Ok(git::GitStatus::Behind(n)) => GitCheckResult::Behind(n),
                Err(_) => GitCheckResult::Failed,
            };
            let _ = tx.send(result);
        });
    }

    pub fn start_status_checks(&mut self) {
        self.status_receivers.clear();

        for (i, item) in self.items.iter().enumerate() {
            if item.kind != ItemKind::Sync {
                continue;
            }

            let id = item.id;
            if id == ItemId::Homebrew {
                let (tx, rx) = mpsc::channel();
                let repo_root = self.repo_root.clone();
                self.status_receivers.push((i, rx));
                thread::spawn(move || {
                    let status = match homebrew::check_homebrew_status(&repo_root) {
                        Ok((installed, total)) => {
                            if installed == total {
                                SyncStatus::Synced
                            } else {
                                SyncStatus::Partial(installed, total)
                            }
                        }
                        Err(_) => SyncStatus::NotSynced,
                    };
                    let _ = tx.send(status);
                });
                continue;
            }

            let (tx, rx) = mpsc::channel();
            let repo_root = self.repo_root.clone();
            self.status_receivers.push((i, rx));
            thread::spawn(move || {
                let status = items::check_item_status(id, &repo_root);
                let _ = tx.send(status);
            });
        }
    }

    fn start_git_pull(&mut self) {
        let (tx, rx) = mpsc::channel();
        let repo_root = self.repo_root.clone();
        self.git_pull_rx = Some(rx);
        self.git_banner = GitBanner::Pulling;

        thread::spawn(move || {
            let _ = tx.send(git::pull(&repo_root));
        });
    }

    fn enter_main(&mut self) {
        self.state = AppState::Main;
    }

    fn enter_main_and_refresh(&mut self) {
        self.state = AppState::Main;
        self.start_status_checks();
    }

    fn start_running(&mut self, idx: usize) {
        let item = &self.items[idx];
        self.command_output.clear();
        self.scroll_offset = 0;

        if item.id == ItemId::Homebrew {
            let cmd = format!(
                "bash {}",
                self.repo_root.join("homebrew/install.sh").display()
            );
            self.runner_rx = Some(runner::spawn_script(&cmd, &self.repo_root));
        } else {
            let id = item.id;
            let repo_root = self.repo_root.clone();
            self.runner_rx = Some(runner::spawn_native(move |tx| {
                items::setup_item(id, &repo_root, tx)
            }));
        }
        self.state = AppState::Running(idx);
    }

    fn start_brew_sync(&mut self) {
        self.brew_untracked.clear();
        self.brew_added = 0;
        self.brew_skipped = 0;
        self.brew_comment = None;
        self.state = AppState::HomebrewSync(BrewSyncState::Loading);

        let (tx, rx) = mpsc::channel();
        let repo_root = self.repo_root.clone();
        self.brew_sync_rx = Some(rx);

        thread::spawn(move || {
            let _ = tx.send(homebrew::find_untracked(&repo_root));
        });
    }

    pub fn is_runner_done(&self) -> bool {
        self.runner_rx.is_none()
    }

    pub fn command_buffer(&self) -> Option<&str> {
        match &self.key_handler.mode {
            KeyMode::Command(s) => Some(s),
            _ => None,
        }
    }

    pub fn search_buffer(&self) -> Option<&str> {
        self.key_handler.search_value()
    }

    pub fn active_search_query(&self) -> Option<&str> {
        if let Some(q) = self.key_handler.search_value() {
            if !q.is_empty() {
                return Some(q);
            }
            return None;
        }
        self.search_query.as_deref()
    }

    fn matching_indices(&self) -> Vec<usize> {
        let query = match self.active_search_query() {
            Some(q) => q.to_lowercase(),
            None => return vec![],
        };
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.label.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect()
    }

    fn jump_to_first_match(&mut self) {
        let matches = self.matching_indices();
        if let Some(&first) = matches.first() {
            self.selected = first;
        }
    }

    fn jump_to_next_match(&mut self) {
        let matches = self.matching_indices();
        if matches.is_empty() {
            return;
        }
        if let Some(&idx) = matches.iter().find(|&&i| i > self.selected) {
            self.selected = idx;
        } else {
            self.selected = matches[0];
        }
    }

    fn jump_to_prev_match(&mut self) {
        let matches = self.matching_indices();
        if matches.is_empty() {
            return;
        }
        if let Some(&idx) = matches.iter().rev().find(|&&i| i < self.selected) {
            self.selected = idx;
        } else {
            self.selected = *matches.last().unwrap();
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if matches!(self.state, AppState::KeyboardLayout(_)) {
            self.handle_keyboard_layout_key(key);
            return;
        }

        let was_searching = matches!(self.key_handler.mode, KeyMode::Search(_));
        let search_text = if was_searching {
            self.key_handler.search_value().map(|s| s.to_string())
        } else {
            None
        };

        let action = self.key_handler.process(key);

        if was_searching {
            let still_searching = matches!(self.key_handler.mode, KeyMode::Search(_));
            if still_searching {
                if matches!(self.state, AppState::Main) {
                    self.jump_to_first_match();
                }
                return;
            }
            match action {
                Action::Confirm => {
                    self.search_query = search_text.filter(|s| !s.is_empty());
                }
                Action::Quit => {
                    self.search_query = None;
                    self.should_quit = true;
                }
                _ => {
                    self.search_query = None;
                }
            }
            return;
        }

        match action {
            Action::Quit => {
                self.should_quit = true;
            }
            Action::Back => match &self.state {
                AppState::Main => {
                    if self.search_query.is_some() {
                        self.search_query = None;
                    } else {
                        self.should_quit = true;
                    }
                }
                AppState::HomebrewSync(BrewSyncState::CommentInput(_)) => {
                    if let AppState::HomebrewSync(BrewSyncState::CommentInput(idx)) = self.state {
                        self.brew_skipped += 1;
                        let next = idx + 1;
                        if next >= self.brew_untracked.len() {
                            self.state = AppState::HomebrewSync(BrewSyncState::Done(
                                self.brew_added,
                                self.brew_skipped,
                            ));
                        } else {
                            self.state = AppState::HomebrewSync(BrewSyncState::Prompting(next));
                        }
                    }
                }
                _ => {
                    self.enter_main();
                }
            },
            Action::ScrollDown => match &self.state {
                AppState::Main => {
                    self.selected = (self.selected + 1) % self.items.len();
                }
                AppState::Running(_) => {
                    if self.scroll_offset + 1 < self.command_output.len() {
                        self.scroll_offset += 1;
                    }
                }
                _ => {}
            },
            Action::ScrollUp => match &self.state {
                AppState::Main => {
                    if self.selected == 0 {
                        self.selected = self.items.len() - 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                AppState::Running(_) => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
                _ => {}
            },
            Action::ScrollTop => match &self.state {
                AppState::Main => {
                    self.selected = 0;
                }
                AppState::Running(_) => {
                    self.scroll_offset = 0;
                }
                _ => {}
            },
            Action::ScrollBottom => match &self.state {
                AppState::Main => {
                    self.selected = self.items.len() - 1;
                }
                AppState::Running(_) => {
                    self.scroll_offset = self.command_output.len().saturating_sub(1);
                }
                _ => {}
            },
            Action::Confirm => match &self.state {
                AppState::Main => {
                    let item = &self.items[self.selected];
                    match item.id {
                        ItemId::KeyboardLayout => {
                            if self.cached_keymap.is_none() {
                                self.cached_keymap = keymap::parse_keymap(&self.repo_root).ok();
                            }
                            self.state = AppState::KeyboardLayout(0);
                        }
                        ItemId::HomebrewSync => {
                            self.start_brew_sync();
                        }
                        ItemId::CorneFlash => {}
                        _ => {
                            self.start_running(self.selected);
                        }
                    }
                }
                AppState::Running(_) => {
                    if self.runner_rx.is_none() {
                        self.enter_main_and_refresh();
                    }
                }
                AppState::HomebrewSync(BrewSyncState::Done(_, _)) => {
                    self.enter_main();
                }
                _ => {}
            },
            Action::Yes => match &self.state {
                AppState::Main if matches!(self.git_banner, GitBanner::Behind(_)) => {
                    self.start_git_pull();
                }
                AppState::HomebrewSync(BrewSyncState::Prompting(idx)) => {
                    let idx = *idx;
                    let pkg = &self.brew_untracked[idx];
                    let _ = homebrew::add_to_install_sh(
                        &self.repo_root,
                        &pkg.name,
                        &pkg.kind,
                        None,
                    );
                    self.brew_added += 1;
                    let next = idx + 1;
                    if next >= self.brew_untracked.len() {
                        self.state = AppState::HomebrewSync(BrewSyncState::Done(
                            self.brew_added,
                            self.brew_skipped,
                        ));
                    } else {
                        self.state = AppState::HomebrewSync(BrewSyncState::Prompting(next));
                    }
                }
                _ => {}
            },
            Action::No => match &self.state {
                AppState::Main if matches!(self.git_banner, GitBanner::Behind(_)) => {
                    self.git_banner = GitBanner::UpToDate;
                }
                AppState::Main if self.search_query.is_some() => {
                    self.jump_to_next_match();
                }
                AppState::HomebrewSync(BrewSyncState::Prompting(idx)) => {
                    let idx = *idx;
                    self.brew_skipped += 1;
                    let next = idx + 1;
                    if next >= self.brew_untracked.len() {
                        self.state = AppState::HomebrewSync(BrewSyncState::Done(
                            self.brew_added,
                            self.brew_skipped,
                        ));
                    } else {
                        self.state = AppState::HomebrewSync(BrewSyncState::Prompting(next));
                    }
                }
                _ => {}
            },
            Action::CharInput('c') => {
                if let AppState::HomebrewSync(BrewSyncState::Prompting(idx)) = self.state {
                    self.key_handler.enter_text_input();
                    self.state = AppState::HomebrewSync(BrewSyncState::CommentInput(idx));
                }
            }
            Action::CharInput('\n') => {
                if let AppState::HomebrewSync(BrewSyncState::CommentInput(idx)) = self.state {
                    let comment = self.brew_comment.take();
                    let pkg = &self.brew_untracked[idx];
                    let _ = homebrew::add_to_install_sh(
                        &self.repo_root,
                        &pkg.name,
                        &pkg.kind,
                        comment.as_deref(),
                    );
                    self.brew_added += 1;
                    let next = idx + 1;
                    if next >= self.brew_untracked.len() {
                        self.state = AppState::HomebrewSync(BrewSyncState::Done(
                            self.brew_added,
                            self.brew_skipped,
                        ));
                    } else {
                        self.state = AppState::HomebrewSync(BrewSyncState::Prompting(next));
                    }
                }
            }
            Action::CharInput('/') => {
                if matches!(self.state, AppState::Main) {
                    self.key_handler.enter_search();
                    self.search_query = None;
                }
            }
            Action::SearchPrev => {
                if matches!(self.state, AppState::Main) && self.search_query.is_some() {
                    self.jump_to_prev_match();
                }
            }
            Action::Tab | Action::NumberKey(_) | Action::CharInput(_) | Action::None => {}
        }
    }

    fn handle_keyboard_layout_key(&mut self, key: KeyEvent) {
        use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

        if matches!(self.key_handler.mode, KeyMode::Command(_)) {
            let action = self.key_handler.process(key);
            match action {
                Action::Quit => {
                    self.highlight_ticks = [0; 42];
                    self.enter_main();
                }
                _ => {}
            }
            return;
        }

        if key.kind == KeyEventKind::Press {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                self.should_quit = true;
                return;
            }
            match key.code {
                KeyCode::Char(':') => {
                    self.key_handler.mode = KeyMode::Command(String::new());
                    return;
                }
                KeyCode::Tab => {
                    if let AppState::KeyboardLayout(ref mut idx) = self.state {
                        *idx += 1;
                        self.highlight_ticks = [0; 42];
                    }
                    return;
                }
                _ => {}
            }
        }

        self.update_highlights(key);
    }

    fn update_highlights(&mut self, key: KeyEvent) {
        use crossterm::event::KeyEventKind;

        let keymap = match &self.cached_keymap {
            Some(km) => km,
            None => return,
        };

        let layer_idx = match &self.state {
            AppState::KeyboardLayout(idx) => *idx % keymap.layers.len().max(1),
            _ => return,
        };

        let labels = keymap::keycode_to_labels(key.code, key.modifiers);
        if labels.is_empty() {
            return;
        }

        match key.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                let layer = &keymap.layers[layer_idx];
                let positions = keymap::find_positions(layer, &labels);

                if let Some(target) = keymap::layer_target_at(layer, &positions) {
                    let clamped = target % keymap.layers.len().max(1);
                    if let AppState::KeyboardLayout(ref mut idx) = self.state {
                        *idx = clamped;
                    }
                    self.highlight_ticks = [0; 42];
                    for pos in &positions {
                        self.highlight_ticks[*pos] = 8;
                    }
                } else if positions.is_empty() {
                    if let Some(new_layer) =
                        keymap::detect_layer(keymap, layer_idx, key.code, key.modifiers)
                    {
                        let base_layer = &keymap.layers[layer_idx];
                        for (pos, target) in base_layer.layer_targets.iter().enumerate() {
                            if *target == Some(new_layer) {
                                self.highlight_ticks[pos] = 8;
                            }
                        }

                        if let AppState::KeyboardLayout(ref mut idx) = self.state {
                            *idx = new_layer;
                        }
                        let new_positions =
                            keymap::find_positions(&keymap.layers[new_layer], &labels);
                        for pos in new_positions {
                            self.highlight_ticks[pos] = 8;
                        }
                    }
                } else {
                    for pos in positions {
                        self.highlight_ticks[pos] = 8;
                    }
                }
            }
            KeyEventKind::Release => {
                let layer = &keymap.layers[layer_idx];
                let positions = keymap::find_positions(layer, &labels);
                if let Some(_) = keymap::layer_target_at(layer, &positions) {
                    if let AppState::KeyboardLayout(ref mut idx) = self.state {
                        *idx = 0;
                    }
                    self.highlight_ticks = [0; 42];
                } else {
                    for pos in &positions {
                        self.highlight_ticks[*pos] = 1;
                    }
                    if positions.is_empty() {
                        for l in &keymap.layers {
                            for pos in keymap::find_positions(l, &labels) {
                                self.highlight_ticks[pos] = 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.spinner_tick += 1;

        for tick in self.highlight_ticks.iter_mut() {
            if *tick > 0 {
                *tick -= 1;
            }
        }

        if let AppState::KeyboardLayout(layer_idx) = &self.state {
            if *layer_idx != 0 {
                if let Some(km) = &self.cached_keymap {
                    let base = &km.layers[0];
                    for (pos, target) in base.layer_targets.iter().enumerate() {
                        if *target == Some(*layer_idx) {
                            self.highlight_ticks[pos] = 2;
                        }
                    }
                }
            }
        }

        if let Some(rx) = self.git_check_rx.take() {
            match rx.try_recv() {
                Ok(GitCheckResult::UpToDate) => {
                    self.git_banner = GitBanner::UpToDate;
                }
                Ok(GitCheckResult::Behind(n)) => {
                    self.git_banner = GitBanner::Behind(n);
                }
                Ok(GitCheckResult::Failed) => {
                    self.git_banner = GitBanner::Failed;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.git_check_rx = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.git_banner = GitBanner::Failed;
                }
            }
        }

        if let Some(rx) = self.git_pull_rx.take() {
            match rx.try_recv() {
                Ok(Ok(_)) => {
                    self.git_banner = GitBanner::UpToDate;
                    self.start_status_checks();
                }
                Ok(Err(_)) => {
                    self.git_banner = GitBanner::Failed;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.git_pull_rx = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.git_banner = GitBanner::Failed;
                }
            }
        }

        let mut completed = Vec::new();
        for (i, (item_idx, rx)) in self.status_receivers.iter().enumerate() {
            if let Ok(status) = rx.try_recv() {
                self.items[*item_idx].status = status;
                completed.push(i);
            }
        }
        for i in completed.into_iter().rev() {
            self.status_receivers.remove(i);
        }

        if let Some(rx) = &self.runner_rx {
            loop {
                match rx.try_recv() {
                    Ok(RunnerMsg::Line(line)) => {
                        self.command_output.push(line);
                        self.scroll_offset = self.command_output.len().saturating_sub(1);
                    }
                    Ok(RunnerMsg::Done(Ok(()))) => {
                        self.command_output.push(String::new());
                        self.command_output
                            .push("\u{2714} Complete. Press Enter to go back.".to_string());
                        self.scroll_offset = self.command_output.len().saturating_sub(1);
                        self.runner_rx = None;
                        break;
                    }
                    Ok(RunnerMsg::Done(Err(e))) => {
                        self.command_output.push(String::new());
                        self.command_output
                            .push(format!("\u{2718} Error: {e}"));
                        self.scroll_offset = self.command_output.len().saturating_sub(1);
                        self.runner_rx = None;
                        break;
                    }
                    Err(_) => break,
                }
            }
        }

        if let Some(rx) = self.brew_sync_rx.take() {
            match rx.try_recv() {
                Ok(Ok(untracked)) => {
                    if untracked.is_empty() {
                        self.state = AppState::HomebrewSync(BrewSyncState::Done(0, 0));
                    } else {
                        self.brew_untracked = untracked;
                        self.state = AppState::HomebrewSync(BrewSyncState::Prompting(0));
                    }
                }
                Ok(Err(e)) => {
                    self.state = AppState::Error(format!("brew sync failed: {e}"));
                }
                Err(mpsc::TryRecvError::Empty) => {
                    self.brew_sync_rx = Some(rx);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.state = AppState::Error("brew sync channel disconnected".to_string());
                }
            }
        }

        if let AppState::HomebrewSync(BrewSyncState::CommentInput(_)) = &self.state {
            if let Some(value) = self.key_handler.text_input_value() {
                self.brew_comment = Some(value.to_string());
            }
        }
    }

    pub fn launch_corne_flash(&self) -> bool {
        matches!(
            self.items.get(self.selected),
            Some(item) if item.id == ItemId::CorneFlash
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_app() -> App {
        let mut app = App {
            state: AppState::Main,
            key_handler: KeyHandler::new(),
            items: MenuItem::all(),
            selected: 0,
            should_quit: false,
            spinner_tick: 0,
            repo_root: PathBuf::from("/tmp/test-dev-env"),
            command_output: Vec::new(),
            scroll_offset: 0,
            brew_untracked: Vec::new(),
            brew_added: 0,
            brew_skipped: 0,
            brew_comment: None,
            git_check_rx: None,
            git_pull_rx: None,
            status_receivers: Vec::new(),
            runner_rx: None,
            brew_sync_rx: None,
            cached_keymap: None,
            highlight_ticks: [0; 42],
            git_banner: GitBanner::UpToDate,
            search_query: None,
        };
        for item in &mut app.items {
            if item.kind == ItemKind::Sync {
                item.status = SyncStatus::Synced;
            }
        }
        app
    }

    #[test]
    fn initial_state_is_main() {
        let app = test_app();
        assert!(matches!(app.state, AppState::Main));
    }

    #[test]
    fn items_have_correct_count() {
        let app = test_app();
        assert_eq!(app.items.len(), 10);
    }

    #[test]
    fn navigate_down() {
        let mut app = test_app();
        assert_eq!(app.selected, 0);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('j'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.selected, 1);
    }

    #[test]
    fn navigate_up_at_top_wraps() {
        let mut app = test_app();
        assert_eq!(app.selected, 0);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('k'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.selected, app.items.len() - 1);
    }

    #[test]
    fn navigate_to_bottom() {
        let mut app = test_app();

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('G'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.selected, app.items.len() - 1);
    }

    #[test]
    fn navigate_to_top() {
        let mut app = test_app();
        app.selected = 5;

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('g'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn esc_on_main_quits() {
        let mut app = test_app();

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Esc,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert!(app.should_quit);
    }

    #[test]
    fn yes_on_git_behind_starts_pull() {
        let mut app = test_app();
        app.git_banner = GitBanner::Behind(3);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('y'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.git_banner, GitBanner::Pulling);
    }

    #[test]
    fn no_on_git_behind_dismisses() {
        let mut app = test_app();
        app.git_banner = GitBanner::Behind(3);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('n'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert_eq!(app.git_banner, GitBanner::UpToDate);
    }

    #[test]
    fn enter_on_keyboard_layout_switches() {
        let mut app = test_app();
        app.state = AppState::Main;
        app.selected = app
            .items
            .iter()
            .position(|i| i.id == ItemId::KeyboardLayout)
            .unwrap();

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Enter,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert!(matches!(app.state, AppState::KeyboardLayout(0)));
    }

    #[test]
    fn tab_cycles_keyboard_layers() {
        let mut app = test_app();
        app.state = AppState::KeyboardLayout(0);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Tab,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert!(matches!(app.state, AppState::KeyboardLayout(1)));
    }

    #[test]
    fn number_keys_highlight_in_keyboard_layout() {
        let mut app = test_app();
        app.state = AppState::KeyboardLayout(0);

        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('3'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert!(matches!(app.state, AppState::KeyboardLayout(0)));
    }

    #[test]
    fn colon_q_quits() {
        let mut app = test_app();
        app.state = AppState::Main;

        let colon = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char(':'),
            crossterm::event::KeyModifiers::NONE,
        );
        let q = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('q'),
            crossterm::event::KeyModifiers::NONE,
        );
        let enter = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Enter,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(colon);
        app.handle_key(q);
        app.handle_key(enter);
        assert!(app.should_quit);
    }

    #[test]
    fn launch_corne_flash_when_selected() {
        let mut app = test_app();
        app.selected = app
            .items
            .iter()
            .position(|i| i.id == ItemId::CorneFlash)
            .unwrap();
        assert!(app.launch_corne_flash());
    }

    #[test]
    fn launch_corne_flash_false_when_other_selected() {
        let mut app = test_app();
        app.selected = 0;
        assert!(!app.launch_corne_flash());
    }

    #[test]
    fn slash_enters_search_mode() {
        let mut app = test_app();
        let key = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('/'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(key);
        assert!(matches!(app.key_handler.mode, KeyMode::Search(_)));
    }

    #[test]
    fn search_jumps_to_match() {
        let mut app = test_app();
        app.selected = 0;

        let slash = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('/'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(slash);

        for c in "tmux".chars() {
            let key = crossterm::event::KeyEvent::new(
                crossterm::event::KeyCode::Char(c),
                crossterm::event::KeyModifiers::NONE,
            );
            app.handle_key(key);
        }

        let tmux_idx = app
            .items
            .iter()
            .position(|i| i.label == "Tmux")
            .unwrap();
        assert_eq!(app.selected, tmux_idx);
    }

    #[test]
    fn search_enter_persists_query() {
        let mut app = test_app();
        let slash = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('/'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(slash);

        let t = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('t'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(t);

        let enter = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Enter,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(enter);

        assert_eq!(app.search_query, Some("t".to_string()));
        assert!(matches!(app.key_handler.mode, KeyMode::Normal));
    }

    #[test]
    fn search_esc_clears_query() {
        let mut app = test_app();
        let slash = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('/'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(slash);

        let t = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('t'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(t);

        let esc = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Esc,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(esc);

        assert_eq!(app.search_query, None);
        assert!(!app.should_quit);
    }

    #[test]
    fn esc_clears_search_before_quit() {
        let mut app = test_app();
        app.search_query = Some("test".to_string());

        let esc = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Esc,
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(esc);
        assert_eq!(app.search_query, None);
        assert!(!app.should_quit);

        app.handle_key(esc);
        assert!(app.should_quit);
    }

    #[test]
    fn n_jumps_to_next_match() {
        let mut app = test_app();
        app.search_query = Some("brew".to_string());
        app.selected = 0;

        let n = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('n'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(n);

        let brew_sync_idx = app
            .items
            .iter()
            .position(|i| i.label == "Homebrew Sync")
            .unwrap();
        assert_eq!(app.selected, brew_sync_idx);
    }

    #[test]
    fn n_wraps_around() {
        let mut app = test_app();
        app.search_query = Some("brew".to_string());
        let brew_sync_idx = app
            .items
            .iter()
            .position(|i| i.label == "Homebrew Sync")
            .unwrap();
        app.selected = brew_sync_idx;

        let n = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('n'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(n);

        assert_eq!(app.selected, 0);
    }

    #[test]
    fn shift_n_jumps_to_prev_match() {
        let mut app = test_app();
        app.search_query = Some("brew".to_string());
        let brew_sync_idx = app
            .items
            .iter()
            .position(|i| i.label == "Homebrew Sync")
            .unwrap();
        app.selected = brew_sync_idx;

        let n = crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char('N'),
            crossterm::event::KeyModifiers::NONE,
        );
        app.handle_key(n);

        assert_eq!(app.selected, 0);
    }

    #[test]
    fn search_case_insensitive() {
        let mut app = test_app();
        app.search_query = Some("FISH".to_string());
        let matches = app.matching_indices();
        let fish_idx = app
            .items
            .iter()
            .position(|i| i.label == "Fish")
            .unwrap();
        assert!(matches.contains(&fish_idx));
    }
}
