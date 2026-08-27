# Dashboard Enhancement Design

## Overview

Enhance the dev-env TUI dashboard with system usage metrics, Claude Code stats, and service monitoring. Two views: a compact right panel on the main menu, and a full-width dedicated dashboard view.

## Views

### Compact Panel (Main View)

Shown when terminal width >= 70 cols. Rendered as a right column separated by a vertical border. Contains:

- CPU/MEM/DSK/SWP gauges (12-char bars, color-coded: green <50%, yellow 50-80%, red >80%)
- CPU sparkline (20 samples, ~40s history, colored per-character)
- Uptime (Xd Xh Xm format)
- Battery percent + charging status
- Claude Code sessions: total, today, this week

### Full Dashboard View

New `AppState::Dashboard` state, accessible via "System Dashboard" menu item in the Actions section. Two-column grid layout.

**Left column:**
- Resources: CPU/MEM/DSK/SWP gauges with sizes (e.g., "6.2 / 16.0 GB"), CPU temp, load average (1/5/15 min)
- CPU sparkline (full width)
- System: uptime, battery
- Claude Code: session stats

**Right column:**
- Network & I/O: Wi-Fi SSID + signal, network throughput (up/down), disk I/O (read/write)
- Bluetooth: connected devices
- Services: tmux sessions, listening ports, docker containers (running/stopped), homebrew package counts

## Architecture

### New Module: `src/system.rs`

Structs:
- `SystemMetrics` -- CPU usage, mem total/used, swap total/used, disk total/used, uptime, battery percent/charging
- `DashboardMetrics` -- extends SystemMetrics with: cpu temp, load avg, wifi ssid/signal, net throughput up/down, disk io read/write, bluetooth devices, tmux sessions, listening ports, docker running/stopped counts, brew formula/cask counts
- `ClaudeStats` -- total sessions, today count, week count
- `MetricsMsg` enum -- System(SystemMetrics), Dashboard(DashboardMetrics), Claude(ClaudeStats)

### Data Collection

Background thread sends `MetricsMsg` via `mpsc::channel` to the App. The app polls in `tick()` via `try_recv()`, same pattern as existing git/status checks.

**Continuous (every 2s):**
- `sysinfo` crate: CPU, MEM, SWP, DSK, uptime
- `pmset -g batt`: battery

**On-demand (when entering dashboard view, then refresh every 5s):**
- CPU temp: `sudo powermetrics` or parsing from IOKit via sysctl
- Load average: `sysctl -n vm.loadavg`
- Wi-Fi: `/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I`
- Network throughput: `netstat -ib` delta between samples
- Disk I/O: `iostat -c 2 -w 1`
- Listening ports: `lsof -iTCP -sTCP:LISTEN -P -n`
- Docker: `docker ps --format json`
- Tmux: `tmux list-sessions`
- Bluetooth: `system_profiler SPBluetoothDataType`
- Brew: `brew list --formula | wc -l` and `brew list --cask | wc -l`

**Once (on startup):**
- Claude stats: walk `~/.claude/projects/**/*.jsonl`, count files and filter by mtime

### App State Changes

New fields on `App`:
- `system_metrics: Option<SystemMetrics>`
- `dashboard_metrics: Option<DashboardMetrics>`
- `claude_stats: Option<ClaudeStats>`
- `cpu_history: Vec<f64>` (capped at 20 entries for compact, 40 for full dashboard)
- `metrics_rx: Option<mpsc::Receiver<MetricsMsg>>`
- `dashboard_rx: Option<mpsc::Receiver<MetricsMsg>>` (for on-demand dashboard metrics)

New `AppState::Dashboard` variant. New `ItemId::Dashboard` in the Actions menu.

### UI Changes

`render_main()` splits horizontally into left (menu) and right (compact panel) when width >= 70. New `render_dashboard_panel()` for the compact view. New `render_dashboard()` for the full view with two-column grid layout.

### Dependencies

Add `sysinfo = "0.33"` to Cargo.toml.

## Navigation

- Main view: "System Dashboard" menu item in Actions section
- Full dashboard: `Esc` to go back, `:q` to quit, `j/k` to scroll if content overflows
- Compact panel: no interaction, display only

## Error Handling

All metric collection is best-effort. If a command fails or isn't available (e.g., no Docker installed, no battery on desktop Mac), that section shows "n/a" or is omitted. No crashes from missing data.
