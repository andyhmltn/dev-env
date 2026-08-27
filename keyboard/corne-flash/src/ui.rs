use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::{App, AppState};

const SPINNER: &[char] = &['\u{280B}', '\u{2819}', '\u{2839}', '\u{2838}', '\u{283C}', '\u{2834}', '\u{2826}', '\u{2827}', '\u{2807}', '\u{280F}'];

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
    let step = app.state.step_number();
    let label = app.state.label();

    let title_left = " Corne Flash ";
    let title_right = if step > 0 {
        format!(" {label} [{step}/6] ")
    } else {
        format!(" {label} ")
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
        AppState::WaitLeftHalf | AppState::WaitRightHalf => {
            vec![("Esc", "quit"), ("s", "skip"), (":q", "quit")]
        }
        AppState::Error(_) => {
            vec![("Esc", "quit"), ("r", "retry"), (":q", "quit")]
        }
        AppState::Done => {
            vec![("Esc", "quit"), (":q", "quit")]
        }
        _ => {
            vec![("Esc", "quit"), (":q", "quit")]
        }
    };

    let content = if let Some(cmd) = app.command_buffer() {
        Line::from(vec![
            Span::styled(":", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(cmd.to_string(), Style::default().fg(Color::White)),
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
        AppState::Fetching => render_fetching(f, app, area),
        AppState::Downloading => render_downloading(f, app, area),
        AppState::WaitLeftHalf => render_wait_half(f, app, area, "LEFT", "keys 0 + 5"),
        AppState::FlashingLeft => render_flashing(f, app, area, "LEFT"),
        AppState::WaitRightHalf => render_wait_half(f, app, area, "RIGHT", "keys 6 + 11"),
        AppState::FlashingRight => render_flashing(f, app, area, "RIGHT"),
        AppState::Done => render_done(f, app, area),
        AppState::Error(msg) => render_error(f, app, area, msg),
    }
}

fn vertical_center(area: Rect, content_height: u16) -> Rect {
    let pad = area.height.saturating_sub(content_height) / 2;
    Rect::new(area.x, area.y + pad, area.width, content_height.min(area.height))
}

fn render_fetching(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            format!("  {} Fetching latest firmware build...", spinner(app.spinner_tick)),
            Style::default().fg(Color::Yellow),
        )),
    ];

    let centered = vertical_center(area, 1);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, centered);
}

fn render_downloading(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    if let Some(info) = &app.run_info {
        let short_sha = if info.sha.len() >= 7 {
            &info.sha[..7]
        } else {
            &info.sha
        };
        let short_date = if info.created_at.len() >= 10 {
            &info.created_at[..10]
        } else {
            &info.created_at
        };

        lines.push(Line::from(vec![
            Span::styled("  Commit  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                short_sha,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Date    ", Style::default().fg(Color::DarkGray)),
            Span::styled(short_date, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Title   ", Style::default().fg(Color::DarkGray)),
            Span::styled(&info.title, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        format!("  {} Downloading firmware...", spinner(app.spinner_tick)),
        Style::default().fg(Color::Yellow),
    )));

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, centered);
}

fn render_wait_half(f: &mut Frame, app: &App, area: Rect, side: &str, combo: &str) {
    let lines = vec![
        Line::from(Span::styled(
            format!("Press bootloader combo on {side} half"),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            combo.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            format!("{} Watching for NICENANO volume...", spinner(app.spinner_tick)),
            Style::default().fg(Color::Yellow),
        )),
    ];

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, centered);
}

fn render_flashing(f: &mut Frame, app: &App, area: Rect, side: &str) {
    let uf2_name = match side {
        "LEFT" => app
            .left_uf2
            .as_ref()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string()),
        _ => app
            .right_uf2
            .as_ref()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string()),
    };

    let lines = vec![
        Line::from(Span::styled(
            format!("{side} half detected!"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  File    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                uf2_name.unwrap_or_default(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Target  ", Style::default().fg(Color::DarkGray)),
            Span::styled("raw block device", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "\u{2714} Firmware written successfully",
            Style::default().fg(Color::Green),
        )),
    ];

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, centered);
}

fn render_done(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(Span::styled(
            "\u{2714} Both halves flashed successfully!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if let Some(info) = &app.run_info {
        let short_sha = if info.sha.len() >= 7 {
            &info.sha[..7]
        } else {
            &info.sha
        };
        lines.push(Line::from(vec![
            Span::styled("  Firmware  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{short_sha} ({0})", info.title),
                Style::default().fg(Color::White),
            ),
        ]));
    }

    if let Some(t) = &app.left_flash_time {
        lines.push(Line::from(vec![
            Span::styled("  Left      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("flashed at {t}"),
                Style::default().fg(Color::Green),
            ),
        ]));
    }

    if let Some(t) = &app.right_flash_time {
        lines.push(Line::from(vec![
            Span::styled("  Right     ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("flashed at {t}"),
                Style::default().fg(Color::Green),
            ),
        ]));
    }

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(paragraph, centered);
}

fn render_error(f: &mut Frame, _app: &App, area: Rect, msg: &str) {
    let lines = vec![
        Line::from(Span::styled(
            "Error",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            msg.to_string(),
            Style::default().fg(Color::Red),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'r' to retry or ':q' to quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let centered = vertical_center(area, lines.len() as u16);
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, centered);
}
