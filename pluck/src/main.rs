use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

const HISTORY_LINES: &str = "-2000";
const MIN_TOKEN_LEN: usize = 5;

#[derive(Clone, Copy)]
enum Mode {
    Words,
    Lines,
}

impl Mode {
    fn next(self) -> Mode {
        match self {
            Mode::Words => Mode::Lines,
            Mode::Lines => Mode::Words,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Mode::Words => "words",
            Mode::Lines => "lines",
        }
    }
}

enum Outcome {
    Copy(String),
    Insert(String),
    Cycle(String),
    Cancel,
}

fn fail(msg: &str) -> ! {
    eprintln!("pluck: {msg}");
    eprintln!("press enter to close");
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    std::process::exit(1);
}

fn tmux(args: &[&str]) -> String {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .unwrap_or_else(|e| fail(&format!("failed to run tmux: {e}")));
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        fail(&format!("tmux {} failed: {}", args.join(" "), stderr.trim()));
    }
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn capture_window(trigger_pane: &str) -> String {
    let mut text = String::new();
    let panes = tmux(&["list-panes", "-t", trigger_pane, "-F", "#{pane_id}"]);
    for pane in panes.lines() {
        if pane != trigger_pane {
            text.push_str(&tmux(&["capture-pane", "-pJ", "-S", HISTORY_LINES, "-t", pane]));
        }
    }
    text.push_str(&tmux(&["capture-pane", "-pJ", "-S", HISTORY_LINES, "-t", trigger_pane]));
    text
}

fn trim_token(token: &str) -> &str {
    let stripped = token.trim_matches(|c| {
        matches!(
            c,
            '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '\'' | '"' | '`' | ',' | ';'
        )
    });
    stripped.trim_end_matches(['.', ':'])
}

fn extract(text: &str, mode: Mode) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for line in text.lines().rev() {
        match mode {
            Mode::Words => {
                for token in line.split_whitespace() {
                    let token = trim_token(token);
                    if token.len() >= MIN_TOKEN_LEN && seen.insert(token.to_owned()) {
                        result.push(token.to_owned());
                    }
                }
            }
            Mode::Lines => {
                let line = line.trim();
                if line.len() >= MIN_TOKEN_LEN && seen.insert(line.to_owned()) {
                    result.push(line.to_owned());
                }
            }
        }
    }
    result
}

fn run_fzf(candidates: &[String], mode: Mode, query: &str) -> Outcome {
    let header = format!("enter=copy  tab=insert  ctrl-f=filter [{}]", mode.name());
    let mut child = Command::new("fzf")
        .args([
            "--multi",
            "--print-query",
            "--tiebreak=index",
            "--no-info",
            "--expect=tab,ctrl-f",
            "--header",
            &header,
            "--query",
            query,
        ])
        .env_remove("FZF_DEFAULT_OPTS")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| fail(&format!("failed to run fzf: {e}")));

    match child.stdin.take() {
        Some(mut stdin) => {
            let _ = stdin.write_all(candidates.join("\n").as_bytes());
        }
        None => fail("fzf stdin unavailable"),
    }

    let output = child
        .wait_with_output()
        .unwrap_or_else(|e| fail(&format!("fzf failed: {e}")));
    if !matches!(output.status.code(), Some(0) | Some(1)) {
        return Outcome::Cancel;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let typed_query = lines.next().unwrap_or("").to_owned();
    let key = lines.next().unwrap_or("");
    let selections: Vec<&str> = lines.collect();
    let selection = match mode {
        Mode::Words => selections.join(" "),
        Mode::Lines => selections.join("\n"),
    };

    if key == "ctrl-f" {
        return Outcome::Cycle(typed_query);
    }
    if selection.is_empty() {
        return Outcome::Cancel;
    }
    if key == "tab" {
        Outcome::Insert(selection)
    } else {
        Outcome::Copy(selection)
    }
}

fn copy_to_clipboard(text: &str) {
    tmux(&["set-buffer", "-w", "--", text]);
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| fail(&format!("failed to run pbcopy: {e}")));
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }
    let _ = child.wait();
}

fn insert_into_pane(pane: &str, text: &str) {
    tmux(&["send-keys", "-t", pane, "-l", "--", text]);
}

fn main() {
    let pane = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => tmux(&["display-message", "-p", "#{pane_id}"])
            .trim()
            .to_owned(),
    };
    let text = capture_window(&pane);
    let mut mode = Mode::Words;
    let mut query = String::new();
    loop {
        let candidates = extract(&text, mode);
        match run_fzf(&candidates, mode, &query) {
            Outcome::Copy(selection) => {
                copy_to_clipboard(&selection);
                break;
            }
            Outcome::Insert(selection) => {
                insert_into_pane(&pane, &selection);
                break;
            }
            Outcome::Cycle(typed_query) => {
                query = typed_query;
                mode = mode.next();
            }
            Outcome::Cancel => break,
        }
    }
}
