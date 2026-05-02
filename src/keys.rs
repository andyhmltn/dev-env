use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
    Back,
    ScrollUp,
    ScrollDown,
    ScrollTop,
    ScrollBottom,
    Confirm,
    Yes,
    No,
    Tab,
    NumberKey(u8),
    CharInput(char),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyMode {
    Normal,
    Command(String),
    TextInput(String),
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
            KeyMode::TextInput(_) => self.process_text_input(key),
        }
    }

    pub fn enter_text_input(&mut self) {
        self.mode = KeyMode::TextInput(String::new());
    }

    pub fn text_input_value(&self) -> Option<&str> {
        match &self.mode {
            KeyMode::TextInput(s) => Some(s),
            _ => None,
        }
    }

    fn process_normal(&mut self, key: KeyEvent) -> Action {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Action::Quit;
        }

        match key.code {
            KeyCode::Esc => Action::Back,
            KeyCode::Char(':') => {
                self.mode = KeyMode::Command(String::new());
                Action::None
            }
            KeyCode::Char('j') => Action::ScrollDown,
            KeyCode::Char('k') => Action::ScrollUp,
            KeyCode::Char('g') => Action::ScrollTop,
            KeyCode::Char('G') => Action::ScrollBottom,
            KeyCode::Char('y') => Action::Yes,
            KeyCode::Char('n') => Action::No,
            KeyCode::Char('c') => Action::CharInput('c'),
            KeyCode::Enter => Action::Confirm,
            KeyCode::Tab => Action::Tab,
            KeyCode::Char(c @ '0'..='9') => Action::NumberKey(c as u8 - b'0'),
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

    fn process_text_input(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Enter => {
                self.mode = KeyMode::Normal;
                Action::CharInput('\n')
            }
            KeyCode::Esc => {
                self.mode = KeyMode::Normal;
                Action::Back
            }
            KeyCode::Char(c) => {
                if let KeyMode::TextInput(ref mut s) = self.mode {
                    s.push(c);
                }
                Action::None
            }
            KeyCode::Backspace => {
                if let KeyMode::TextInput(ref mut s) = self.mode {
                    s.pop();
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
    fn escape_goes_back() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key(KeyCode::Esc)), Action::Back);
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
    fn ctrl_c_quits() {
        let mut handler = KeyHandler::new();
        let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(handler.process(event), Action::Quit);
    }

    #[test]
    fn command_mode_escape_returns_to_normal() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        assert_eq!(handler.process(key(KeyCode::Esc)), Action::None);
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn number_keys() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char('0')), Action::NumberKey(0));
        assert_eq!(handler.process(key_char('5')), Action::NumberKey(5));
    }

    #[test]
    fn tab_key() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key(KeyCode::Tab)), Action::Tab);
    }

    #[test]
    fn yes_no_keys() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char('y')), Action::Yes);
        assert_eq!(handler.process(key_char('n')), Action::No);
    }

    #[test]
    fn enter_confirms() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key(KeyCode::Enter)), Action::Confirm);
    }

    #[test]
    fn c_key_char_input() {
        let mut handler = KeyHandler::new();
        assert_eq!(handler.process(key_char('c')), Action::CharInput('c'));
    }

    #[test]
    fn command_mode_backspace_on_content() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        handler.process(key_char('a'));
        handler.process(key_char('b'));
        assert_eq!(handler.mode, KeyMode::Command("ab".to_string()));
        handler.process(key(KeyCode::Backspace));
        assert_eq!(handler.mode, KeyMode::Command("a".to_string()));
    }

    #[test]
    fn command_mode_backspace_empty_exits() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        handler.process(key(KeyCode::Backspace));
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn colon_quit_also_quits() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        for c in "quit".chars() {
            handler.process(key_char(c));
        }
        assert_eq!(handler.process(key(KeyCode::Enter)), Action::Quit);
    }

    #[test]
    fn unknown_command_is_noop() {
        let mut handler = KeyHandler::new();
        handler.process(key_char(':'));
        handler.process(key_char('x'));
        assert_eq!(handler.process(key(KeyCode::Enter)), Action::None);
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn text_input_mode() {
        let mut handler = KeyHandler::new();
        handler.enter_text_input();
        assert!(matches!(handler.mode, KeyMode::TextInput(_)));

        handler.process(key_char('h'));
        handler.process(key_char('i'));
        assert_eq!(handler.text_input_value(), Some("hi"));

        handler.process(key(KeyCode::Backspace));
        assert_eq!(handler.text_input_value(), Some("h"));
    }

    #[test]
    fn text_input_enter_returns_to_normal() {
        let mut handler = KeyHandler::new();
        handler.enter_text_input();
        handler.process(key_char('a'));
        let action = handler.process(key(KeyCode::Enter));
        assert_eq!(action, Action::CharInput('\n'));
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn text_input_escape_returns_to_normal() {
        let mut handler = KeyHandler::new();
        handler.enter_text_input();
        handler.process(key_char('a'));
        let action = handler.process(key(KeyCode::Esc));
        assert_eq!(action, Action::Back);
        assert_eq!(handler.mode, KeyMode::Normal);
    }

    #[test]
    fn ignores_key_release_events() {
        let mut handler = KeyHandler::new();
        let mut event = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        event.kind = KeyEventKind::Release;
        assert_eq!(handler.process(event), Action::None);
    }

    #[test]
    fn all_number_keys() {
        let mut handler = KeyHandler::new();
        for i in 0..=9u8 {
            let c = (b'0' + i) as char;
            assert_eq!(handler.process(key_char(c)), Action::NumberKey(i));
        }
    }
}
