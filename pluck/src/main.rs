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
        fail(&format!(
            "tmux {} failed: {}",
            args.join(" "),
            stderr.trim()
        ));
    }
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn capture_window(trigger_pane: &str) -> String {
    let mut text = String::new();
    let panes = tmux(&["list-panes", "-t", trigger_pane, "-F", "#{pane_id}"]);
    for pane in panes.lines() {
        if pane != trigger_pane {
            text.push_str(&tmux(&[
                "capture-pane",
                "-pJ",
                "-S",
                HISTORY_LINES,
                "-t",
                pane,
            ]));
        }
    }
    text.push_str(&tmux(&[
        "capture-pane",
        "-pJ",
        "-S",
        HISTORY_LINES,
        "-t",
        trigger_pane,
    ]));
    text
}

fn is_wrap_junk(c: char) -> bool {
    matches!(
        c,
        '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '\'' | '"' | '`' | ',' | ';'
    )
}

fn is_trailing_junk(c: char) -> bool {
    is_wrap_junk(c) || matches!(c, '.' | ':')
}

/// Chars that commonly appear glued to the front of a useful token
/// (shell prompts, diff markers, hex/number prefixes, bullets, etc.).
fn is_leading_junk(c: char) -> bool {
    is_wrap_junk(c)
        || c.is_ascii_digit()
        || matches!(
            c,
            '.' | ':'
                | '='
                | '|'
                | '&'
                | '*'
                | '!'
                | '?'
                | '#'
                | '@'
                | '$'
                | '%'
                | '^'
                | '~'
                | '/'
                | '\\'
                | '+'
                | '-'
        )
}

fn trim_token(token: &str) -> &str {
    let stripped = token.trim_matches(is_wrap_junk);
    stripped.trim_end_matches(is_trailing_junk)
}

/// Produce all useful search variants of a whitespace-separated token by
/// progressively peeling junk characters off the front and back. For
/// something like `>>0defAbcd` we emit `>>0defAbcd`, `>0defAbcd`,
/// `0defAbcd`, `defAbcd`, so any of them is fzf-searchable.
fn token_variants(token: &str) -> Vec<String> {
    let base = trim_token(token);
    if base.is_empty() {
        return Vec::new();
    }
    let mut variants: Vec<String> = Vec::new();
    let mut local: HashSet<String> = HashSet::new();
    let push = |variants: &mut Vec<String>, local: &mut HashSet<String>, s: &str| {
        if !s.is_empty() && local.insert(s.to_owned()) {
            variants.push(s.to_owned());
        }
    };
    push(&mut variants, &mut local, base);

    // Peel from the left one char at a time.
    let mut s = base;
    while let Some(c) = s.chars().next() {
        if !is_leading_junk(c) {
            break;
        }
        s = &s[c.len_utf8()..];
        let trimmed = s.trim_end_matches(is_trailing_junk);
        push(&mut variants, &mut local, trimmed);
    }

    // Peel from the right one char at a time.
    let mut s = base;
    while let Some(c) = s.chars().last() {
        if !is_trailing_junk(c) {
            break;
        }
        s = &s[..s.len() - c.len_utf8()];
        push(&mut variants, &mut local, s);
    }

    variants
}

fn word_slices(line: &str) -> Vec<&str> {
    let mut slices = Vec::new();
    let mut at_word_start = true;

    for (index, c) in line.char_indices() {
        if c.is_whitespace() {
            at_word_start = true;
        } else if at_word_start {
            slices.push(line[index..].trim_end());
            at_word_start = false;
        }
    }

    slices
}

fn extract(text: &str, mode: Mode) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for line in text.lines().rev() {
        match mode {
            Mode::Words => {
                for token in line.split_whitespace() {
                    for variant in token_variants(token) {
                        if variant.len() >= MIN_TOKEN_LEN && seen.insert(variant.clone()) {
                            result.push(variant);
                        }
                    }
                }
                for slice in word_slices(line) {
                    if slice.len() >= MIN_TOKEN_LEN && seen.insert(slice.to_owned()) {
                        result.push(slice.to_owned());
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

fn starts_with_query(candidate: &str, query: &str) -> bool {
    if query.chars().any(char::is_uppercase) {
        candidate.starts_with(query)
    } else {
        candidate.to_lowercase().starts_with(&query.to_lowercase())
    }
}

fn rank_candidates<'a>(candidates: &'a [String], query: &str) -> Vec<&'a str> {
    let mut prefix_matches = Vec::new();
    let mut other_matches = Vec::new();

    for candidate in candidates {
        if starts_with_query(candidate, query) {
            prefix_matches.push(candidate.as_str());
        } else {
            other_matches.push(candidate.as_str());
        }
    }

    prefix_matches.extend(other_matches);
    prefix_matches
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\\"'\\\"'"))
}

fn rank_from_file(path: &str, query: &str) {
    let candidates = std::fs::read_to_string(path)
        .unwrap_or_else(|e| fail(&format!("failed to read candidates: {e}")))
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let ranked = rank_candidates(&candidates, query);
    let output = ranked.join("\n");
    let _ = std::io::stdout().write_all(output.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::{extract, rank_candidates, Mode};

    #[test]
    fn words_include_slices_starting_at_each_word() {
        let candidates = extract(
            "To continue this session, run codex resume 01a01ff8-3c04-70e3-8337-47e53dc5f374",
            Mode::Words,
        );

        assert!(candidates
            .iter()
            .any(|candidate| { candidate == "codex resume 01a01ff8-3c04-70e3-8337-47e53dc5f374" }));
        assert!(candidates.iter().any(|candidate| {
            candidate == "run codex resume 01a01ff8-3c04-70e3-8337-47e53dc5f374"
        }));
    }

    #[test]
    fn query_prefixes_rank_before_other_matches() {
        let candidates = vec![
            "run codex resume session".to_owned(),
            "codex resume session".to_owned(),
            "another codex resume session".to_owned(),
        ];
        let ranked = rank_candidates(&candidates, "codex resu");

        assert_eq!(
            ranked,
            [
                "codex resume session",
                "run codex resume session",
                "another codex resume session"
            ]
        );
    }
}

fn run_fzf(candidates: &[String], mode: Mode, query: &str) -> Outcome {
    let header = format!("enter=copy  tab=insert  ctrl-f=filter [{}]", mode.name());
    let candidate_file =
        std::env::temp_dir().join(format!("pluck-{}.candidates", std::process::id()));
    std::fs::write(&candidate_file, candidates.join("\n"))
        .unwrap_or_else(|e| fail(&format!("failed to save candidates: {e}")));
    let executable = std::env::current_exe()
        .unwrap_or_else(|e| fail(&format!("failed to find pluck executable: {e}")));
    let reload = format!(
        "change:reload:{} --rank {} {{q}}",
        shell_quote(&executable.to_string_lossy()),
        shell_quote(&candidate_file.to_string_lossy())
    );
    let mut child = Command::new("fzf")
        .args([
            "--multi",
            "--print-query",
            "--no-sort",
            "--bind",
            &reload,
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
            let initial_candidates = rank_candidates(candidates, query).join("\n");
            let _ = stdin.write_all(initial_candidates.as_bytes());
        }
        None => fail("fzf stdin unavailable"),
    }

    let output = child
        .wait_with_output()
        .unwrap_or_else(|e| fail(&format!("fzf failed: {e}")));
    let _ = std::fs::remove_file(&candidate_file);
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
    let mut args = std::env::args();
    let _ = args.next();
    let first_arg = args.next();
    if matches!(first_arg.as_deref(), Some("--rank")) {
        let path = args
            .next()
            .unwrap_or_else(|| fail("candidate file is required"));
        let query = args.next().unwrap_or_default();
        rank_from_file(&path, &query);
        return;
    }

    let pane = match first_arg {
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
