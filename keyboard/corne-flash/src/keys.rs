use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
    ScrollUp,
    ScrollDown,
    ScrollTop,
    ScrollBottom,
    Skip,
    Retry,
    Confirm,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyMode {
    Normal,
    Command(String),
}

pub struct KeyHandler {
    pub mode: KeyMode,
}

impl KeyHandler {
    pub fn new() -> Self {
        Self {
            mode: KeyMode::Normal,
        }
    }

    pub fn process(&mut self, key: KeyEvent) -> Action {
        if key.kind != KeyEventKind::Press {
            return Action::None;
        }

        match &self.mode {
            KeyMode::Normal => self.process_normal(key),
            KeyMode::Command(_) => self.process_command(key),
        }
    }

    fn process_normal(&mut self, key: KeyEvent) -> Action {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Action::Quit;
        }

        match key.code {
            KeyCode::Esc => Action::Quit,
            KeyCode::Char(':') => {
                self.mode = KeyMode::Command(String::new());
                Action::None
            }
            KeyCode::Char('j') => Action::ScrollDown,
            KeyCode::Char('k') => Action::ScrollUp,
            KeyCode::Char('g') => Action::ScrollTop,
            KeyCode::Char('G') => Action::ScrollBottom,
            KeyCode::Char('s') => Action::Skip,
            KeyCode::Char('r') => Action::Retry,
            KeyCode::Enter => Action::Confirm,
            _ => Action::None,
        }
    }

    fn process_command(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Enter => {
                let cmd = match &self.mode {
                    KeyMode::Command(s) => s.clone(),
                    _ => String::new(),
                };
                self.mode = KeyMode::Normal;
                match cmd.as_str() {
                    "q" | "quit" => Action::Quit,
                    _ => Action::None,
                }
            }
            KeyCode::Esc => {
                self.mode = KeyMode::Normal;
                Action::None
            }
            KeyCode::Char(c) => {
                if let KeyMode::Command(ref mut s) = self.mode {
                    s.push(c);
                }
                Action::None
            }
            KeyCode::Backspace => {
                if let KeyMode::Command(ref mut s) = self.mode {
                    if s.is_empty() {
                        self.mode = KeyMode::Normal;
                    } else {
                        s.pop();
                    }
                }
                Action::None
            }
            _ => Action::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn key_char(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
    }

    #[test]
    fn escape_quits() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key(KeyCode::Esc)), Action::Quit);
    }

    #[test]
    fn colon_q_quits() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char(':')), Action::None);
        assert_eq!(handler.mode, KeyMode::Command(String::new()));
        assert_eq!(handler.process(key_char('q')), Action::None);
        assert_eq!(handler.process(key(KeyCode::Enter)), Action::Quit);
    }

    #[test]
    fn vim_navigation() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char('j')), Action::ScrollDown);
        assert_eq!(handler.process(key_char('k')), Action::ScrollUp);
        assert_eq!(handler.process(key_char('g')), Action::ScrollTop);
        assert_eq!(handler.process(key_char('G')), Action::ScrollBottom);
    }

    #[test]
    fn skip_and_retry() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char('s')), Action::Skip);
        assert_eq!(handler.process(key_char('r')), Action::Retry);
    }

    #[test]
    fn command_mode_escape_returns_to_normal() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        assert_eq!(handler.process(key(KeyCode::Esc)), Action::None);
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn command_mode_backspace_empty_returns_to_normal() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        handler.process(key(KeyCode::Backspace));
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn ctrl_c_quits() {
        let mut handler = KeyHandler::new();
        let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(handler.process(event), Action::Quit);
    }

    #[test]
    fn unknown_command_is_noop() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        handler.process(key_char('x'));
        assert_eq!(handler.process(key(KeyCode::Enter)), Action::None);
        assert_eq!(handler.mode, KeyMode::Normal);
    }
}
