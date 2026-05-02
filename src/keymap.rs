use std::path::Path;

use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub keys: Vec<String>,
    pub layer_targets: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
pub struct Keymap {
    pub layers: Vec<Layer>,
}

pub fn keycode_to_labels(code: KeyCode, modifiers: KeyModifiers) -> Vec<String> {
    let base = match code {
        KeyCode::Char(' ') => Some("SPC".to_string()),
        KeyCode::Char(c) => {
            let upper = c.to_ascii_uppercase().to_string();
            Some(upper)
        }
        KeyCode::Backspace => Some("BSP".to_string()),
        KeyCode::Enter => Some("ENT".to_string()),
        KeyCode::Left => Some("\u{2190}".to_string()),
        KeyCode::Right => Some("\u{2192}".to_string()),
        KeyCode::Up => Some("\u{2191}".to_string()),
        KeyCode::Down => Some("\u{2193}".to_string()),
        KeyCode::Home => Some("HOM".to_string()),
        KeyCode::End => Some("END".to_string()),
        KeyCode::Tab => Some("TAB".to_string()),
        KeyCode::Esc => Some("ESC".to_string()),
        KeyCode::Delete => Some("DEL".to_string()),
        _ => None,
    };

    let Some(base) = base else {
        return vec![];
    };

    let mut labels = Vec::new();

    let has_ctrl = modifiers.contains(KeyModifiers::CONTROL);
    let has_alt = modifiers.contains(KeyModifiers::ALT);
    let has_super = modifiers.contains(KeyModifiers::SUPER);

    if has_ctrl && has_alt {
        labels.push(format!("M-{base}"));
    } else if has_ctrl {
        labels.push(format!("C-{base}"));
    } else if has_alt {
        labels.push(format!("A-{base}"));
    } else if has_super {
        labels.push(format!("G-{base}"));
    }

    labels.push(base);

    if let KeyCode::Char(c) = code {
        let sym = match c {
            '!' => Some("!"),
            '@' => Some("@"),
            '#' => Some("#"),
            '$' => Some("$"),
            '%' => Some("%"),
            '^' => Some("^"),
            '&' => Some("&"),
            '*' => Some("*"),
            '(' => Some("("),
            ')' => Some(")"),
            '-' => Some("-"),
            '_' => Some("_"),
            '=' => Some("="),
            '+' => Some("+"),
            '[' => Some("["),
            ']' => Some("]"),
            '{' => Some("{"),
            '}' => Some("}"),
            '\\' => Some("\\"),
            '|' => Some("|"),
            ';' => Some(";"),
            ':' => Some(":"),
            '\'' => Some("'"),
            '"' => Some("\""),
            ',' => Some(","),
            '<' => Some("<"),
            '.' => Some("."),
            '>' => Some(">"),
            '/' => Some("/"),
            '?' => Some("?"),
            '`' => Some("`"),
            '~' => Some("~"),
            _ => None,
        };
        if let Some(s) = sym {
            let s = s.to_string();
            if !labels.contains(&s) {
                labels.push(s);
            }
        }
    }

    labels
}

pub fn find_positions(layer: &Layer, labels: &[String]) -> Vec<usize> {
    let mut positions = Vec::new();
    for (i, key) in layer.keys.iter().enumerate() {
        if key.is_empty() {
            continue;
        }
        for label in labels {
            if key == label {
                positions.push(i);
                break;
            }
        }
    }
    positions
}

pub fn layer_target_at(layer: &Layer, positions: &[usize]) -> Option<usize> {
    for pos in positions {
        if let Some(Some(target)) = layer.layer_targets.get(*pos) {
            return Some(*target);
        }
    }
    None
}

pub fn detect_layer(keymap: &Keymap, current: usize, code: KeyCode, modifiers: KeyModifiers) -> Option<usize> {
    let labels = keycode_to_labels(code, modifiers);
    if labels.is_empty() {
        return None;
    }

    let current_idx = current % keymap.layers.len().max(1);
    let current_positions = find_positions(&keymap.layers[current_idx], &labels);
    if !current_positions.is_empty() {
        return None;
    }

    for (i, layer) in keymap.layers.iter().enumerate() {
        if i == current_idx {
            continue;
        }
        if !find_positions(layer, &labels).is_empty() {
            return Some(i);
        }
    }

    None
}

pub fn parse_keymap(repo_root: &Path) -> Result<Keymap> {
    let content = std::fs::read_to_string(repo_root.join("config/corne.keymap"))
        .context("reading corne.keymap")?;

    let mut layers = Vec::new();
    let mut i = 0;
    let lines: Vec<&str> = content.lines().collect();

    while i < lines.len() {
        let line = lines[i].trim();

        if line.ends_with("_layer {") || line.ends_with("_layer{") {
            let name = line
                .trim_end_matches('{')
                .trim()
                .trim_end_matches("_layer")
                .trim()
                .to_string();

            let mut bindings_str = String::new();
            let mut in_bindings = false;

            i += 1;
            while i < lines.len() {
                let inner = lines[i].trim();
                if inner.starts_with("bindings = <") {
                    in_bindings = true;
                    let after = inner.trim_start_matches("bindings = <");
                    bindings_str.push_str(after);
                    bindings_str.push(' ');
                } else if in_bindings {
                    if inner.contains(">;") {
                        let before = inner.trim_end_matches(">;").trim_end_matches('>');
                        bindings_str.push_str(before);
                        break;
                    }
                    bindings_str.push_str(inner);
                    bindings_str.push(' ');
                }
                if inner == "};" {
                    break;
                }
                i += 1;
            }

            let (keys, layer_targets) = parse_bindings(&bindings_str);
            layers.push(Layer {
                name,
                keys,
                layer_targets,
            });
        }

        i += 1;
    }

    Ok(Keymap { layers })
}

fn parse_bindings(raw: &str) -> (Vec<String>, Vec<Option<usize>>) {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();

    for ch in raw.chars() {
        match ch {
            '&' => {
                if !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                }
                current = String::from("&");
            }
            _ => {
                current.push(ch);
            }
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }

    let keys = tokens.iter().map(|t| binding_to_label(t)).collect();
    let targets = tokens.iter().map(|t| binding_layer_target(t)).collect();
    (keys, targets)
}

fn binding_layer_target(binding: &str) -> Option<usize> {
    let parts: Vec<&str> = binding.split_whitespace().collect();
    match parts.first()? {
        &"&mo" | &"&to" => parts.get(1)?.parse().ok(),
        &"&lt" | &"&lt_enter" => parts.get(1)?.parse().ok(),
        _ => None,
    }
}

fn binding_to_label(binding: &str) -> String {
    let parts: Vec<&str> = binding.split_whitespace().collect();
    if parts.is_empty() {
        return String::new();
    }

    let behavior = parts[0];
    match behavior {
        "&kp" => {
            if parts.len() >= 2 {
                key_label(parts[1])
            } else {
                String::new()
            }
        }
        "&mt" => {
            if parts.len() >= 3 {
                key_label(parts[2])
            } else {
                String::new()
            }
        }
        "&lt" | "&lt_enter" => {
            if parts.len() >= 3 {
                key_label(parts[2])
            } else if parts.len() >= 2 {
                format!("L{}", parts[1])
            } else {
                String::new()
            }
        }
        "&mo" => {
            if parts.len() >= 2 {
                format!("L{}", parts[1])
            } else {
                String::new()
            }
        }
        "&to" => {
            if parts.len() >= 2 {
                format!("TO{}", parts[1])
            } else {
                String::new()
            }
        }
        "&trans" => String::new(),
        "&none" => "---".to_string(),
        "&caps_word" => "CW".to_string(),
        "&bootloader" => "BOOT".to_string(),
        "&bt" => {
            if parts.len() >= 2 {
                match parts[1] {
                    "BT_SEL" => {
                        if parts.len() >= 3 {
                            format!("BT{}", parts[2])
                        } else {
                            "BT".to_string()
                        }
                    }
                    "BT_CLR" => "CLR".to_string(),
                    "BT_PRV" => "PRV".to_string(),
                    "BT_NXT" => "NXT".to_string(),
                    other => other.to_string(),
                }
            } else {
                "BT".to_string()
            }
        }
        _ => {
            if let Some(label) = macro_label(behavior) {
                label.to_string()
            } else {
                behavior
                    .trim_start_matches('&')
                    .chars()
                    .take(5)
                    .collect()
            }
        }
    }
}

fn key_label(key: &str) -> String {
    if key.starts_with("LA(LG(") || key.starts_with("LC(LA(") || key.starts_with("LA(LS(") {
        let inner = key
            .replace("LA(", "")
            .replace("LG(", "")
            .replace("LC(", "")
            .replace("LS(", "")
            .replace("RS(", "")
            .replace(')', "");
        return format!("M-{}", key_label(&inner));
    }

    if let Some(inner) = key.strip_prefix("LG(") {
        let inner = inner.trim_end_matches(')');
        return format!("G-{}", key_label(inner));
    }
    if let Some(inner) = key.strip_prefix("LA(") {
        let inner = inner.trim_end_matches(')');
        return format!("A-{}", key_label(inner));
    }
    if let Some(inner) = key.strip_prefix("LC(") {
        let inner = inner.trim_end_matches(')');
        return format!("C-{}", key_label(inner));
    }
    if let Some(inner) = key.strip_prefix("LS(") {
        let inner = inner.trim_end_matches(')');
        return format!("S-{}", key_label(inner));
    }

    match key {
        "SPACE" => "SPC".to_string(),
        "ENTER" | "RET" => "ENT".to_string(),
        "BSPC" => "BSP".to_string(),
        "TAB" => "TAB".to_string(),
        "ESC" => "ESC".to_string(),
        "DEL" | "DELETE" => "DEL".to_string(),
        "LSHFT" | "RSHFT" => "SFT".to_string(),
        "LCTRL" | "RCTRL" => "CTL".to_string(),
        "LALT" | "RALT" => "ALT".to_string(),
        "LGUI" | "RGUI" => "GUI".to_string(),
        "LEFT" => "\u{2190}".to_string(),
        "RIGHT" => "\u{2192}".to_string(),
        "UP" => "\u{2191}".to_string(),
        "DOWN" => "\u{2193}".to_string(),
        "HOME" => "HOM".to_string(),
        "END" => "END".to_string(),
        "DOT" => ".".to_string(),
        "COMMA" => ",".to_string(),
        "SEMI" => ";".to_string(),
        "COLON" => ":".to_string(),
        "SQT" => "'".to_string(),
        "DQT" => "\"".to_string(),
        "FSLH" => "/".to_string(),
        "BSLH" => "\\".to_string(),
        "MINUS" => "-".to_string(),
        "UNDER" => "_".to_string(),
        "EQUAL" => "=".to_string(),
        "PLUS" => "+".to_string(),
        "STAR" | "KP_MULTIPLY" => "*".to_string(),
        "EXCL" => "!".to_string(),
        "AT" => "@".to_string(),
        "HASH" => "#".to_string(),
        "DLLR" => "$".to_string(),
        "PRCNT" => "%".to_string(),
        "CARET" => "^".to_string(),
        "AMPS" => "&".to_string(),
        "LPAR" => "(".to_string(),
        "RPAR" => ")".to_string(),
        "LBKT" => "[".to_string(),
        "RBKT" => "]".to_string(),
        "LBRC" => "{".to_string(),
        "RBRC" => "}".to_string(),
        "LT" => "<".to_string(),
        "GT" => ">".to_string(),
        "PIPE" => "|".to_string(),
        "GRAVE" => "`".to_string(),
        "TILDE" => "~".to_string(),
        "QMARK" => "?".to_string(),
        "N0" => "0".to_string(),
        "N1" => "1".to_string(),
        "N2" => "2".to_string(),
        "N3" => "3".to_string(),
        "N4" => "4".to_string(),
        "N5" => "5".to_string(),
        "N6" => "6".to_string(),
        "N7" => "7".to_string(),
        "N8" => "8".to_string(),
        "N9" => "9".to_string(),
        other => other.to_string(),
    }
}

fn macro_label(name: &str) -> Option<&'static str> {
    match name {
        "&m_sp_otd" => Some("-otd"),
        "&m_esc_w_cr" => Some(":w"),
        "&m_sp_gg" => Some(" gg"),
        "&m_q_bang_cr" => Some(":q!"),
        "&m_sp_ca" => Some(" ca"),
        "&m_sp_dash" => Some(" -"),
        "&m_tmux_z" => Some("^Bz"),
        "&m_tmux_c" => Some("^Bc"),
        "&m_tmux_x" => Some("^Bx"),
        "&m_tmux_copy" => Some("^B["),
        "&m_tmux_l" => Some("^Bl"),
        "&m_tmux_split_h" => Some("^B\""),
        "&m_tmux_split_v" => Some("^B%"),
        "&m_jump_prev_diag" => Some("[g"),
        "&m_jump_next_diag" => Some("]g"),
        "&m_tick_A" => Some("`A"),
        "&m_tick_B" => Some("`B"),
        "&m_tick_C" => Some("`C"),
        "&m_slash_slash" => Some("//"),
        "&m_fat_arrow" => Some("=>"),
        "&m_dotdot_slash" => Some("../"),
        "&m_55_star" => Some("55*"),
        "&m_col_plus_1_col" => Some(":+1:"),
        "&m_sp_gtl" => Some("-gtl"),
        "&m_sp_btl" => Some("-btl"),
        _ => None,
    }
}

#[cfg(test)]
pub fn render_layer_text(layer: &Layer) -> Vec<String> {
    let keys = &layer.keys;
    if keys.len() < 42 {
        return vec![format!("Layer '{}' has {} keys (expected 42)", layer.name, keys.len())];
    }

    let row1_l: Vec<&str> = (0..6).map(|i| keys[i].as_str()).collect();
    let row1_r: Vec<&str> = (6..12).map(|i| keys[i].as_str()).collect();
    let row2_l: Vec<&str> = (12..18).map(|i| keys[i].as_str()).collect();
    let row2_r: Vec<&str> = (18..24).map(|i| keys[i].as_str()).collect();
    let row3_l: Vec<&str> = (24..30).map(|i| keys[i].as_str()).collect();
    let row3_r: Vec<&str> = (30..36).map(|i| keys[i].as_str()).collect();
    let thumb_l: Vec<&str> = (36..39).map(|i| keys[i].as_str()).collect();
    let thumb_r: Vec<&str> = (39..42).map(|i| keys[i].as_str()).collect();

    let w = 5;
    let mut lines = Vec::new();

    lines.push(format!(
        " {}  {}",
        row_top_border(6, w),
        row_top_border(6, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_cells(&row1_l, w),
        row_cells(&row1_r, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_mid_border(6, w),
        row_mid_border(6, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_cells(&row2_l, w),
        row_cells(&row2_r, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_mid_border(6, w),
        row_mid_border(6, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_cells(&row3_l, w),
        row_cells(&row3_r, w)
    ));
    lines.push(format!(
        " {}  {}",
        row_bottom_border(6, w),
        row_bottom_border(6, w)
    ));

    let thumb_pad = " ".repeat(w * 3 + 4);
    lines.push(format!(
        " {}{}  {}",
        thumb_pad,
        row_top_border(3, w),
        row_top_border(3, w)
    ));
    lines.push(format!(
        " {}{}  {}",
        thumb_pad,
        row_cells(&thumb_l, w),
        row_cells(&thumb_r, w)
    ));
    lines.push(format!(
        " {}{}  {}",
        thumb_pad,
        row_bottom_border(3, w),
        row_bottom_border(3, w)
    ));

    lines
}

#[cfg(test)]
fn row_top_border(count: usize, w: usize) -> String {
    let cell = "\u{2500}".repeat(w);
    let cells: Vec<String> = (0..count).map(|_| cell.clone()).collect();
    format!("\u{250C}{}\u{2510}", cells.join("\u{252C}"))
}

#[cfg(test)]
fn row_mid_border(count: usize, w: usize) -> String {
    let cell = "\u{2500}".repeat(w);
    let cells: Vec<String> = (0..count).map(|_| cell.clone()).collect();
    format!("\u{251C}{}\u{2524}", cells.join("\u{253C}"))
}

#[cfg(test)]
fn row_bottom_border(count: usize, w: usize) -> String {
    let cell = "\u{2500}".repeat(w);
    let cells: Vec<String> = (0..count).map(|_| cell.clone()).collect();
    format!("\u{2514}{}\u{2518}", cells.join("\u{2534}"))
}

#[cfg(test)]
fn row_cells(keys: &[&str], w: usize) -> String {
    let cells: Vec<String> = keys
        .iter()
        .map(|k| {
            let display = if k.len() > w {
                &k[..w]
            } else {
                k
            };
            let pad_total = w.saturating_sub(display.len());
            let pad_left = pad_total / 2;
            let pad_right = pad_total - pad_left;
            format!(
                "{}{}{}",
                " ".repeat(pad_left),
                display,
                " ".repeat(pad_right)
            )
        })
        .collect();
    format!("\u{2502}{}\u{2502}", cells.join("\u{2502}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_label_simple() {
        assert_eq!(key_label("Q"), "Q");
        assert_eq!(key_label("SPACE"), "SPC");
        assert_eq!(key_label("BSPC"), "BSP");
        assert_eq!(key_label("N1"), "1");
        assert_eq!(key_label("EXCL"), "!");
    }

    #[test]
    fn test_key_label_modifiers() {
        assert_eq!(key_label("LG(C)"), "G-C");
        assert_eq!(key_label("LC(B)"), "C-B");
        assert_eq!(key_label("LA(TAB)"), "A-TAB");
    }

    #[test]
    fn test_binding_to_label() {
        assert_eq!(binding_to_label("&kp Q"), "Q");
        assert_eq!(binding_to_label("&mt LALT TAB"), "TAB");
        assert_eq!(binding_to_label("&lt 3 ESC"), "ESC");
        assert_eq!(binding_to_label("&mo 1"), "L1");
        assert_eq!(binding_to_label("&trans"), "");
        assert_eq!(binding_to_label("&caps_word"), "CW");
    }

    #[test]
    fn test_macro_labels() {
        assert_eq!(binding_to_label("&m_fat_arrow"), "=>");
        assert_eq!(binding_to_label("&m_esc_w_cr"), ":w");
        assert_eq!(binding_to_label("&m_tmux_z"), "^Bz");
    }

    #[test]
    fn test_bt_bindings() {
        assert_eq!(binding_to_label("&bt BT_SEL 0"), "BT0");
        assert_eq!(binding_to_label("&bt BT_CLR"), "CLR");
        assert_eq!(binding_to_label("&bt BT_PRV"), "PRV");
    }

    fn test_layer(name: &str, keys: Vec<String>) -> Layer {
        let len = keys.len();
        Layer {
            name: name.to_string(),
            keys,
            layer_targets: vec![None; len],
        }
    }

    #[test]
    fn test_parse_bindings() {
        let raw = "&kp Q  &kp W  &trans  &mo 1";
        let (labels, targets) = parse_bindings(raw);
        assert_eq!(labels, vec!["Q", "W", "", "L1"]);
        assert_eq!(targets, vec![None, None, None, Some(1)]);
    }

    #[test]
    fn test_render_layer_has_correct_line_count() {
        let keys: Vec<String> = (0..42).map(|i| format!("K{i}")).collect();
        let layer = test_layer("test", keys);
        let lines = render_layer_text(&layer);
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_row_cells_centering() {
        let result = row_cells(&["Q", "AB", "12345"], 5);
        assert!(result.contains("  Q  "));
        assert!(result.contains(" AB  "));
        assert!(result.contains("12345"));
    }

    #[test]
    fn test_to_binding() {
        assert_eq!(binding_to_label("&to 0"), "TO0");
    }

    #[test]
    fn test_none_binding() {
        assert_eq!(binding_to_label("&none"), "---");
    }

    #[test]
    fn test_bootloader_binding() {
        assert_eq!(binding_to_label("&bootloader"), "BOOT");
    }

    #[test]
    fn test_lt_enter_binding() {
        assert_eq!(binding_to_label("&lt_enter 2 RET"), "ENT");
    }

    #[test]
    fn test_all_symbol_keys() {
        assert_eq!(key_label("DOT"), ".");
        assert_eq!(key_label("COMMA"), ",");
        assert_eq!(key_label("SEMI"), ";");
        assert_eq!(key_label("SQT"), "'");
        assert_eq!(key_label("DQT"), "\"");
        assert_eq!(key_label("FSLH"), "/");
        assert_eq!(key_label("BSLH"), "\\");
        assert_eq!(key_label("MINUS"), "-");
        assert_eq!(key_label("UNDER"), "_");
        assert_eq!(key_label("EQUAL"), "=");
        assert_eq!(key_label("PLUS"), "+");
        assert_eq!(key_label("AT"), "@");
        assert_eq!(key_label("HASH"), "#");
        assert_eq!(key_label("DLLR"), "$");
        assert_eq!(key_label("PRCNT"), "%");
        assert_eq!(key_label("CARET"), "^");
        assert_eq!(key_label("AMPS"), "&");
        assert_eq!(key_label("LPAR"), "(");
        assert_eq!(key_label("RPAR"), ")");
        assert_eq!(key_label("LBKT"), "[");
        assert_eq!(key_label("RBKT"), "]");
        assert_eq!(key_label("LBRC"), "{");
        assert_eq!(key_label("RBRC"), "}");
        assert_eq!(key_label("LT"), "<");
        assert_eq!(key_label("GT"), ">");
        assert_eq!(key_label("PIPE"), "|");
        assert_eq!(key_label("GRAVE"), "`");
        assert_eq!(key_label("QMARK"), "?");
    }

    #[test]
    fn test_arrow_keys() {
        assert_eq!(key_label("LEFT"), "\u{2190}");
        assert_eq!(key_label("RIGHT"), "\u{2192}");
        assert_eq!(key_label("UP"), "\u{2191}");
        assert_eq!(key_label("DOWN"), "\u{2193}");
    }

    #[test]
    fn test_number_keys() {
        for i in 0..=9 {
            assert_eq!(key_label(&format!("N{i}")), format!("{i}"));
        }
    }

    #[test]
    fn test_render_layer_too_few_keys() {
        let keys: Vec<String> = (0..10).map(|i| format!("K{i}")).collect();
        let layer = test_layer("short", keys);
        let lines = render_layer_text(&layer);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("expected 42"));
    }

    #[test]
    fn test_all_macro_labels_defined() {
        let macros = vec![
            "&m_sp_otd",
            "&m_esc_w_cr",
            "&m_sp_gg",
            "&m_q_bang_cr",
            "&m_sp_ca",
            "&m_sp_dash",
            "&m_tmux_z",
            "&m_tmux_c",
            "&m_tmux_x",
            "&m_tmux_copy",
            "&m_tmux_l",
            "&m_tmux_split_h",
            "&m_tmux_split_v",
            "&m_jump_prev_diag",
            "&m_jump_next_diag",
            "&m_tick_A",
            "&m_tick_B",
            "&m_tick_C",
            "&m_slash_slash",
            "&m_fat_arrow",
            "&m_dotdot_slash",
            "&m_55_star",
            "&m_col_plus_1_col",
            "&m_sp_gtl",
            "&m_sp_btl",
        ];
        for m in macros {
            assert!(
                macro_label(m).is_some(),
                "missing macro label for {m}"
            );
        }
    }

    #[test]
    fn test_parse_real_keymap() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        if repo.join("config/corne.keymap").exists() {
            let km = parse_keymap(repo).unwrap();
            assert_eq!(km.layers.len(), 8);
            assert_eq!(km.layers[0].name, "default");
            assert_eq!(km.layers[1].name, "fn");
            assert_eq!(km.layers[2].name, "sym");
            assert_eq!(km.layers[3].name, "num");
            assert_eq!(km.layers[4].name, "scrn");
            assert_eq!(km.layers[5].name, "move");
            assert_eq!(km.layers[6].name, "bt");
            assert_eq!(km.layers[7].name, "aero_move");

            for layer in &km.layers {
                assert_eq!(
                    layer.keys.len(),
                    42,
                    "layer '{}' has {} keys",
                    layer.name,
                    layer.keys.len()
                );
            }

            assert_eq!(km.layers[0].keys[1], "Q");
            assert_eq!(km.layers[0].keys[2], "W");
            assert_eq!(km.layers[0].keys[3], "E");
        }
    }

    #[test]
    fn test_border_functions() {
        let top = row_top_border(3, 5);
        assert!(top.starts_with('\u{250C}'));
        assert!(top.ends_with('\u{2510}'));
        assert!(top.contains('\u{252C}'));

        let mid = row_mid_border(3, 5);
        assert!(mid.starts_with('\u{251C}'));
        assert!(mid.ends_with('\u{2524}'));

        let bot = row_bottom_border(3, 5);
        assert!(bot.starts_with('\u{2514}'));
        assert!(bot.ends_with('\u{2518}'));
    }

    #[test]
    fn test_keycode_to_labels_alpha() {
        let labels = keycode_to_labels(KeyCode::Char('a'), KeyModifiers::NONE);
        assert!(labels.contains(&"A".to_string()));
    }

    #[test]
    fn test_keycode_to_labels_space() {
        let labels = keycode_to_labels(KeyCode::Char(' '), KeyModifiers::NONE);
        assert!(labels.contains(&"SPC".to_string()));
    }

    #[test]
    fn test_keycode_to_labels_special() {
        assert!(keycode_to_labels(KeyCode::Backspace, KeyModifiers::NONE).contains(&"BSP".to_string()));
        assert!(keycode_to_labels(KeyCode::Enter, KeyModifiers::NONE).contains(&"ENT".to_string()));
        assert!(keycode_to_labels(KeyCode::Left, KeyModifiers::NONE).contains(&"\u{2190}".to_string()));
    }

    #[test]
    fn test_keycode_to_labels_ctrl() {
        let labels = keycode_to_labels(KeyCode::Char('b'), KeyModifiers::CONTROL);
        assert!(labels.contains(&"C-B".to_string()));
    }

    #[test]
    fn test_keycode_to_labels_symbols() {
        let labels = keycode_to_labels(KeyCode::Char('.'), KeyModifiers::NONE);
        assert!(labels.contains(&".".to_string()));
        let labels = keycode_to_labels(KeyCode::Char('-'), KeyModifiers::NONE);
        assert!(labels.contains(&"-".to_string()));
    }

    #[test]
    fn test_find_positions_basic() {
        let layer = test_layer("test", vec![
            "TAB".to_string(), "Q".to_string(), "W".to_string(), "E".to_string(),
            "R".to_string(), "T".to_string(), "Y".to_string(), "U".to_string(),
            "I".to_string(), "O".to_string(), "P".to_string(), "_".to_string(),
        ]);
        let positions = find_positions(&layer, &["Q".to_string()]);
        assert_eq!(positions, vec![1]);
    }

    #[test]
    fn test_find_positions_empty_for_trans() {
        let layer = test_layer("test", vec!["".to_string(), "Q".to_string()]);
        let positions = find_positions(&layer, &["".to_string()]);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_find_positions_multiple() {
        let layer = test_layer("test", vec!["Q".to_string(), "W".to_string(), "Q".to_string()]);
        let positions = find_positions(&layer, &["Q".to_string()]);
        assert_eq!(positions, vec![0, 2]);
    }

    #[test]
    fn test_detect_layer_stays_on_current() {
        let km = Keymap {
            layers: vec![
                test_layer("base", vec!["Q".to_string()]),
                test_layer("fn", vec!["".to_string()]),
            ],
        };
        let result = detect_layer(&km, 0, KeyCode::Char('q'), KeyModifiers::NONE);
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_layer_switches() {
        let km = Keymap {
            layers: vec![
                test_layer("base", vec!["Q".to_string()]),
                test_layer("fn", vec!["\u{2190}".to_string()]),
            ],
        };
        let result = detect_layer(&km, 0, KeyCode::Left, KeyModifiers::NONE);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_binding_layer_target() {
        assert_eq!(binding_layer_target("&mo 1"), Some(1));
        assert_eq!(binding_layer_target("&lt 3 ESC"), Some(3));
        assert_eq!(binding_layer_target("&lt_enter 2 RET"), Some(2));
        assert_eq!(binding_layer_target("&to 0"), Some(0));
        assert_eq!(binding_layer_target("&kp Q"), None);
        assert_eq!(binding_layer_target("&trans"), None);
    }

    #[test]
    fn test_layer_target_at() {
        let mut layer = test_layer("test", vec!["ESC".to_string(), "Q".to_string(), "L1".to_string()]);
        layer.layer_targets = vec![Some(3), None, Some(1)];

        assert_eq!(layer_target_at(&layer, &[0]), Some(3));
        assert_eq!(layer_target_at(&layer, &[1]), None);
        assert_eq!(layer_target_at(&layer, &[2]), Some(1));
        assert_eq!(layer_target_at(&layer, &[1, 2]), Some(1));
    }

    #[test]
    fn test_parse_bindings_with_layer_targets() {
        let raw = "&lt 3 ESC  &kp Q  &mo 1  &lt_enter 2 RET";
        let (labels, targets) = parse_bindings(raw);
        assert_eq!(labels, vec!["ESC", "Q", "L1", "ENT"]);
        assert_eq!(targets, vec![Some(3), None, Some(1), Some(2)]);
    }

    #[test]
    fn test_real_keymap_layer_targets() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        if repo.join("config/corne.keymap").exists() {
            let km = parse_keymap(repo).unwrap();
            let default = &km.layers[0];
            assert_eq!(default.layer_targets[12], Some(3));
            assert_eq!(default.layer_targets[38], Some(1));
            assert_eq!(default.layer_targets[39], Some(2));
            assert_eq!(default.layer_targets[0], None);
        }
    }
}
