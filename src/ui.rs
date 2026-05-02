use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::{App, AppState, BrewSyncState, GitBanner};
use crate::banner;
use crate::homebrew::PkgKind;
use crate::items::{ItemKind, SyncStatus};
use crate::keys::KeyMode;

const SPINNER: &[char] = &[
    '\u{280B}', '\u{2819}', '\u{2839}', '\u{2838}', '\u{283C}', '\u{2834}', '\u{2826}',
    '\u{2827}', '\u{2807}', '\u{280F}',
];

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(f.area());

    render_header(f, app, chunks[0]);
    render_content(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);
}

fn spinner(tick: usize) -> char {
    SPINNER[tick / 2 % SPINNER.len()]
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title_left = " dev-env ";
    let title_right = match &app.state {
        AppState::Main => match &app.git_banner {
            GitBanner::Checking => format!(" {} checking for updates ", spinner(app.spinner_tick)),
            GitBanner::Behind(n) => format!(" {n} update(s) available -- y pull / n dismiss "),
            GitBanner::Pulling => format!(" {} pulling... ", spinner(app.spinner_tick)),
            GitBanner::UpToDate | GitBanner::Failed => String::new(),
        },
        AppState::Running(idx) => format!(" Running: {} ", app.items[*idx].label),
        AppState::HomebrewSync(_) => " Homebrew Sync ".to_string(),
        AppState::KeyboardLayout(layer) => format!(" Keyboard: Layer {layer} "),
        AppState::Dashboard => " System Dashboard ".to_string(),
        AppState::Error(_) => " Error ".to_string(),
    };

    let block = Block::default()
        .title(Line::from(title_left).left_aligned())
        .title(Line::from(title_right).right_aligned())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .border_type(ratatui::widgets::BorderType::Rounded);

    f.render_widget(block, area);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let hints: Vec<(&str, &str)> = match &app.state {
        AppState::Main => {
            if app.search_query.is_some() {
                vec![
                    ("j/k", "navigate"),
                    ("n/N", "next/prev match"),
                    ("Enter", "select"),
                    ("/", "search"),
                    (":q", "quit"),
                ]
            } else {
                vec![
                    ("j/k", "navigate"),
                    ("Enter", "select"),
                    ("/", "search"),
                    (":q", "quit"),
                ]
            }
        }
        AppState::Running(_) => {
            if app.is_runner_done() {
                vec![("Enter", "back"), ("Esc", "back"), (":q", "quit")]
            } else {
                vec![("j/k", "scroll"), (":q", "quit")]
            }
        }
        AppState::HomebrewSync(BrewSyncState::Loading) => {
            vec![("Esc", "back"), (":q", "quit")]
        }
        AppState::HomebrewSync(BrewSyncState::Prompting(_)) => {
            vec![("y", "add"), ("n", "skip"), ("c", "comment"), ("Esc", "back")]
        }
        AppState::HomebrewSync(BrewSyncState::CommentInput(_)) => {
            vec![("Enter", "confirm"), ("Esc", "cancel")]
        }
        AppState::HomebrewSync(BrewSyncState::Done(_, _)) => {
            vec![("Enter", "back"), ("Esc", "back")]
        }
        AppState::KeyboardLayout(_) => {
            vec![("Tab", "next layer"), (":q", "back")]
        }
        AppState::Dashboard => {
            vec![("Esc", "back"), (":q", "quit")]
        }
        AppState::Error(_) => {
            vec![("Esc", "back"), (":q", "quit")]
        }
    };

    let content = if let Some(cmd) = app.command_buffer() {
        Line::from(vec![
            Span::styled(
                ":",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(cmd.to_string(), Style::default().fg(Color::White)),
            Span::styled("\u{2588}", Style::default().fg(Color::White)),
        ])
    } else if let Some(text) = match &app.key_handler.mode {
        KeyMode::TextInput(s) => Some(s),
        _ => None,
    } {
        Line::from(vec![
            Span::styled(
                "comment: ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(text.to_string(), Style::default().fg(Color::White)),
            Span::styled("\u{2588}", Style::default().fg(Color::White)),
        ])
    } else if let Some(query) = app.search_buffer() {
        Line::from(vec![
            Span::styled(
                "/",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(query.to_string(), Style::default().fg(Color::White)),
            Span::styled("\u{2588}", Style::default().fg(Color::White)),
        ])
    } else {
        let mut spans = Vec::new();
        for (i, (key, desc)) in hints.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("   "));
            }
            spans.push(Span::styled(
                *key,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(*desc, Style::default().fg(Color::DarkGray)));
        }
        Line::from(spans)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .border_type(ratatui::widgets::BorderType::Rounded);

    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);
}

fn render_content(f: &mut Frame, app: &App, area: Rect) {
    match &app.state {
        AppState::Main => render_main(f, app, area),
        AppState::Running(_) => render_running(f, app, area),
        AppState::HomebrewSync(sub) => render_brew_sync(f, app, area, sub),
        AppState::KeyboardLayout(layer) => render_keyboard_layout(f, app, area, *layer),
        AppState::Dashboard => {}
        AppState::Error(msg) => render_error(f, area, msg),
    }
}

fn vertical_center(area: Rect, content_height: u16) -> Rect {
    let pad = area.height.saturating_sub(content_height) / 2;
    Rect::new(
        area.x,
        area.y + pad,
        area.width,
        content_height.min(area.height),
    )
}

fn render_main(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    for line in banner::BANNER.lines() {
        lines.push(Line::from(Span::styled(
            format!("  {line}"),
            Style::default().fg(Color::Cyan),
        )));
    }
    lines.push(Line::from(""));

    let search_query = app.active_search_query();
    let highlight_style = Style::default()
        .fg(Color::Black)
        .bg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let mut prev_kind: Option<ItemKind> = None;
    for (i, item) in app.items.iter().enumerate() {
        if let Some(ref pk) = prev_kind {
            if *pk == ItemKind::Sync && item.kind == ItemKind::Action {
                lines.push(Line::from(""));
            }
        }
        prev_kind = Some(item.kind.clone());

        let cursor = if i == app.selected { ">" } else { " " };
        let is_selected = i == app.selected;

        let (status_icon, status_text, status_color) = match &item.status {
            SyncStatus::Synced => {
                if item.kind == ItemKind::Sync {
                    ("\u{2714}", "synced".to_string(), Color::Green)
                } else {
                    ("", item.description.to_string(), Color::DarkGray)
                }
            }
            SyncStatus::NotSynced => ("\u{2718}", "not synced".to_string(), Color::Red),
            SyncStatus::Partial(done, total) => {
                ("\u{25CB}", format!("{done}/{total}"), Color::Yellow)
            }
            SyncStatus::Checking => {
                let s = spinner(app.spinner_tick);
                render_main_item_checking(&mut lines, cursor, item.label, s, is_selected, search_query, highlight_style);
                continue;
            }
        };

        let cursor_style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let label_style = if is_selected {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let label_spans = styled_label_with_highlight(item.label, search_query, label_style, highlight_style);

        if status_icon.is_empty() {
            let mut spans = vec![
                Span::styled(format!("  {cursor} "), cursor_style),
            ];
            spans.extend(label_spans);
            spans.push(Span::styled(status_text, Style::default().fg(status_color)));
            lines.push(Line::from(spans));
        } else {
            let mut spans = vec![
                Span::styled(format!("  {cursor} "), cursor_style),
                Span::styled(
                    format!("{status_icon} "),
                    Style::default().fg(status_color),
                ),
            ];
            spans.extend(label_spans);
            spans.push(Span::styled(status_text, Style::default().fg(status_color)));
            lines.push(Line::from(spans));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn styled_label_with_highlight(
    label: &str,
    query: Option<&str>,
    base_style: Style,
    highlight_style: Style,
) -> Vec<Span<'static>> {
    let padded = format!("{:<18}", label);

    let query = match query {
        Some(q) if !q.is_empty() => q,
        _ => return vec![Span::styled(padded, base_style)],
    };

    let lower = label.to_lowercase();
    let lower_query = query.to_lowercase();

    if let Some(pos) = lower.find(&lower_query) {
        let end = pos + lower_query.len();
        vec![
            Span::styled(padded[..pos].to_string(), base_style),
            Span::styled(padded[pos..end].to_string(), highlight_style),
            Span::styled(padded[end..].to_string(), base_style),
        ]
    } else {
        vec![Span::styled(padded, base_style)]
    }
}

fn render_main_item_checking(
    lines: &mut Vec<Line>,
    cursor: &str,
    label: &str,
    spinner_char: char,
    is_selected: bool,
    search_query: Option<&str>,
    highlight_style: Style,
) {
    let cursor_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let label_style = if is_selected {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let label_spans = styled_label_with_highlight(label, search_query, label_style, highlight_style);

    let mut spans = vec![
        Span::styled(format!("  {cursor} "), cursor_style),
        Span::styled(
            format!("{spinner_char} "),
            Style::default().fg(Color::Yellow),
        ),
    ];
    spans.extend(label_spans);
    spans.push(Span::styled("checking...", Style::default().fg(Color::DarkGray)));
    lines.push(Line::from(spans));
}

fn render_running(f: &mut Frame, app: &App, area: Rect) {
    let visible_height = area.height as usize;
    let start = app.scroll_offset.saturating_sub(visible_height.saturating_sub(1));
    let end = (start + visible_height).min(app.command_output.len());

    let lines: Vec<Line> = app.command_output[start..end]
        .iter()
        .map(|line| {
            if line.starts_with('\u{2714}') {
                Line::from(Span::styled(
                    format!("  {line}"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ))
            } else if line.starts_with('\u{2718}') {
                Line::from(Span::styled(
                    format!("  {line}"),
                    Style::default().fg(Color::Red),
                ))
            } else {
                Line::from(Span::styled(
                    format!("  {line}"),
                    Style::default().fg(Color::White),
                ))
            }
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_brew_sync(f: &mut Frame, app: &App, area: Rect, sub: &BrewSyncState) {
    match sub {
        BrewSyncState::Loading => {
            let lines = vec![Line::from(Span::styled(
                format!(
                    "  {} Scanning installed packages...",
                    spinner(app.spinner_tick)
                ),
                Style::default().fg(Color::Yellow),
            ))];
            let centered = vertical_center(area, 1);
            let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
            f.render_widget(paragraph, centered);
        }
        BrewSyncState::Prompting(idx) => {
            let pkg = &app.brew_untracked[*idx];
            let kind_label = match pkg.kind {
                PkgKind::Formula => "formula",
                PkgKind::Cask => "cask",
            };
            let total = app.brew_untracked.len();

            let lines = vec![
                Line::from(Span::styled(
                    format!("  Found {} untracked package(s)", total),
                    Style::default().fg(Color::White),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        kind_label,
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(": ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &pkg.name,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(Span::styled(
                    "  Add to install.sh? (y/n/c)",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!("  Progress: {}/{total}", idx + 1),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(Span::styled(
                    format!(
                        "  Added: {}  Skipped: {}",
                        app.brew_added, app.brew_skipped
                    ),
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            let centered = vertical_center(area, lines.len() as u16);
            let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
            f.render_widget(paragraph, centered);
        }
        BrewSyncState::CommentInput(idx) => {
            let pkg = &app.brew_untracked[*idx];
            let kind_label = match pkg.kind {
                PkgKind::Formula => "formula",
                PkgKind::Cask => "cask",
            };

            let lines = vec![
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(kind_label, Style::default().fg(Color::DarkGray)),
                    Span::styled(": ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &pkg.name,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "  Enter a comment for this package:",
                    Style::default().fg(Color::Yellow),
                )),
            ];

            let centered = vertical_center(area, lines.len() as u16);
            let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
            f.render_widget(paragraph, centered);
        }
        BrewSyncState::Done(added, skipped) => {
            let msg = if *added == 0 && *skipped == 0 {
                "All packages are already tracked!".to_string()
            } else {
                format!("Done: {added} added, {skipped} skipped")
            };

            let lines = vec![
                Line::from(Span::styled(
                    format!("  \u{2714} {msg}"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Press Enter to go back",
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            let centered = vertical_center(area, 3);
            let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
            f.render_widget(paragraph, centered);
        }
    }
}

fn render_keyboard_layout(f: &mut Frame, app: &App, area: Rect, layer_idx: usize) {
    match &app.cached_keymap {
        Some(km) => {
            let idx = layer_idx % km.layers.len().max(1);
            let layer = &km.layers[idx];
            let keys = &layer.keys;
            let hl = &app.highlight_ticks;

            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line::from(Span::styled(
                format!("  Layer: {}", layer.name),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            if keys.len() >= 42 {
                let w = 5;
                let rows: [(usize, usize, usize, usize); 3] =
                    [(0, 6, 6, 12), (12, 18, 18, 24), (24, 30, 30, 36)];

                for (ri, (ll, lr, rl, rr)) in rows.iter().enumerate() {
                    if ri == 0 {
                        lines.push(grid_border_line(6, 6, w, BorderKind::Top));
                    } else {
                        lines.push(grid_border_line(6, 6, w, BorderKind::Mid));
                    }
                    lines.push(grid_cell_line(keys, hl, *ll..*lr, *rl..*rr, w));
                }
                lines.push(grid_border_line(6, 6, w, BorderKind::Bottom));

                let pad = " ".repeat(w * 3 + 4);
                lines.push(grid_thumb_border_line(&pad, 3, 3, w, BorderKind::Top));
                lines.push(grid_thumb_cell_line(&pad, keys, hl, 36..39, 39..42, w));
                lines.push(grid_thumb_border_line(&pad, 3, 3, w, BorderKind::Bottom));
            }

            lines.push(Line::from(""));

            let mut tab_spans = vec![Span::raw("  ")];
            for (i, l) in km.layers.iter().enumerate() {
                if i > 0 {
                    tab_spans.push(Span::raw("  "));
                }
                if i == idx {
                    tab_spans.push(Span::styled(
                        format!("[{i}] {}", l.name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    tab_spans.push(Span::styled(
                        format!("[{i}] {}", l.name),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
            lines.push(Line::from(tab_spans));

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press keys to highlight. Layer auto-switches on keypress.",
                Style::default().fg(Color::DarkGray),
            )));

            let paragraph = Paragraph::new(lines);
            f.render_widget(paragraph, area);
        }
        None => {
            let lines = vec![Line::from(Span::styled(
                "  Error loading keymap",
                Style::default().fg(Color::Red),
            ))];
            let paragraph = Paragraph::new(lines);
            f.render_widget(paragraph, area);
        }
    }
}

enum BorderKind {
    Top,
    Mid,
    Bottom,
}

fn grid_border_line(left_count: usize, right_count: usize, w: usize, kind: BorderKind) -> Line<'static> {
    let (start, mid, cross, end) = match kind {
        BorderKind::Top => ("\u{250C}", "\u{252C}", "\u{2510}  \u{250C}", "\u{2510}"),
        BorderKind::Mid => ("\u{251C}", "\u{253C}", "\u{2524}  \u{251C}", "\u{2524}"),
        BorderKind::Bottom => ("\u{2514}", "\u{2534}", "\u{2518}  \u{2514}", "\u{2518}"),
    };
    let seg = "\u{2500}".repeat(w);
    let left: Vec<&str> = (0..left_count).map(|_| seg.as_str()).collect();
    let right: Vec<&str> = (0..right_count).map(|_| seg.as_str()).collect();
    let s = format!(
        "  {start}{}{cross}{}{}",
        left.join(mid),
        right.join(mid),
        end,
    );
    Line::from(Span::styled(s, Style::default().fg(Color::DarkGray)))
}

fn grid_cell_line<'a>(
    keys: &[String],
    hl: &[u8; 42],
    left: std::ops::Range<usize>,
    right: std::ops::Range<usize>,
    w: usize,
) -> Line<'a> {
    let border = Style::default().fg(Color::DarkGray);
    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled("  \u{2502}", border));
    for i in left {
        cell_span(&mut spans, &keys[i], hl[i] > 0, w);
        spans.push(Span::styled("\u{2502}", border));
    }
    spans.push(Span::styled("  \u{2502}", border));
    for i in right {
        cell_span(&mut spans, &keys[i], hl[i] > 0, w);
        spans.push(Span::styled("\u{2502}", border));
    }
    Line::from(spans)
}

fn grid_thumb_border_line(pad: &str, left_count: usize, right_count: usize, w: usize, kind: BorderKind) -> Line<'static> {
    let (start, mid, cross, end) = match kind {
        BorderKind::Top => ("\u{250C}", "\u{252C}", "\u{2510}  \u{250C}", "\u{2510}"),
        BorderKind::Mid => ("\u{251C}", "\u{253C}", "\u{2524}  \u{251C}", "\u{2524}"),
        BorderKind::Bottom => ("\u{2514}", "\u{2534}", "\u{2518}  \u{2514}", "\u{2518}"),
    };
    let seg = "\u{2500}".repeat(w);
    let left: Vec<&str> = (0..left_count).map(|_| seg.as_str()).collect();
    let right: Vec<&str> = (0..right_count).map(|_| seg.as_str()).collect();
    let s = format!(
        "  {pad}{start}{}{cross}{}{}",
        left.join(mid),
        right.join(mid),
        end,
    );
    Line::from(Span::styled(s, Style::default().fg(Color::DarkGray)))
}

fn grid_thumb_cell_line<'a>(
    pad: &str,
    keys: &[String],
    hl: &[u8; 42],
    left: std::ops::Range<usize>,
    right: std::ops::Range<usize>,
    w: usize,
) -> Line<'a> {
    let border = Style::default().fg(Color::DarkGray);
    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(format!("  {pad}\u{2502}"), border));
    for i in left {
        cell_span(&mut spans, &keys[i], hl[i] > 0, w);
        spans.push(Span::styled("\u{2502}", border));
    }
    spans.push(Span::styled("  \u{2502}", border));
    for i in right {
        cell_span(&mut spans, &keys[i], hl[i] > 0, w);
        spans.push(Span::styled("\u{2502}", border));
    }
    Line::from(spans)
}

fn char_width(s: &str) -> usize {
    s.chars().count()
}

fn cell_span(spans: &mut Vec<Span<'static>>, label: &str, highlighted: bool, w: usize) {
    let cw = char_width(label);
    let display = if cw > w {
        label.chars().take(w).collect::<String>()
    } else {
        label.to_string()
    };
    let cw = char_width(&display);
    let pad_total = w.saturating_sub(cw);
    let pad_left = pad_total / 2;
    let pad_right = pad_total - pad_left;
    let text = format!(
        "{}{}{}",
        " ".repeat(pad_left),
        display,
        " ".repeat(pad_right)
    );

    let style = if highlighted {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else if label.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };

    spans.push(Span::styled(text, style));
}

fn render_error(f: &mut Frame, area: Rect, msg: &str) {
    let lines = vec![
        Line::from(Span::styled(
            "Error",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(msg.to_string(), Style::default().fg(Color::Red))),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to go back",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, centered);
}
