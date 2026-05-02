# Dashboard Enhancement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add system usage metrics, Claude Code stats, and service monitoring to the dev-env TUI dashboard via a compact right panel on the main view and a dedicated full-width dashboard view.

**Architecture:** A new `system.rs` module handles all metric collection via background threads using `mpsc::channel` (matching existing patterns in `git.rs`/`app.rs`). Core metrics (CPU/MEM/DSK/SWP/battery) refresh every 2s continuously. Dashboard-only metrics (wifi/ports/docker/tmux/bluetooth/brew/net/diskio/temp/load) refresh on-demand when the dashboard view is open. All command output parsing is separated into pure functions for testability.

**Tech Stack:** Rust, ratatui 0.29, crossterm 0.28, sysinfo 0.33 (new dep), macOS shell commands for hardware-specific metrics.

**Spec:** `docs/superpowers/specs/2026-05-02-dashboard-enhancement-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `Cargo.toml` | Modify | Add `sysinfo` dependency |
| `src/system.rs` | Create | All metric types, parsing functions, collection functions, background collector |
| `src/main.rs` | Modify | Add `mod system;` |
| `src/items.rs` | Modify | Add `ItemId::Dashboard` variant and menu item |
| `src/app.rs` | Modify | Add metric state fields, `AppState::Dashboard`, tick polling, dashboard lifecycle |
| `src/ui.rs` | Modify | Compact panel rendering, full dashboard view rendering, gauge/sparkline helpers |

---

### Task 1: Add sysinfo dependency and create system.rs with types

**Files:**
- Modify: `Cargo.toml`
- Create: `src/system.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add sysinfo to Cargo.toml**

Add to `[dependencies]`:
```toml
sysinfo = "0.33"
```

- [ ] **Step 2: Create src/system.rs with all types**

```rust
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use sysinfo::{Components, Disks, Networks, System};

pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub mem_total_bytes: u64,
    pub mem_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub uptime_secs: u64,
    pub battery_percent: Option<f32>,
    pub battery_charging: bool,
}

pub struct DashboardMetrics {
    pub cpu_temp: Option<f32>,
    pub load_avg: [f64; 3],
    pub wifi_ssid: Option<String>,
    pub wifi_signal_dbm: Option<i32>,
    pub net_up_bytes_sec: u64,
    pub net_down_bytes_sec: u64,
    pub disk_read_mb_sec: Option<f64>,
    pub disk_write_mb_sec: Option<f64>,
    pub listening_ports: Vec<PortInfo>,
    pub docker_running: usize,
    pub docker_stopped: usize,
    pub docker_available: bool,
    pub tmux_sessions: Vec<String>,
    pub bluetooth_devices: Vec<BtDevice>,
    pub brew_formulae: usize,
    pub brew_casks: usize,
}

pub struct PortInfo {
    pub port: u16,
    pub process: String,
}

pub struct BtDevice {
    pub name: String,
    pub connected: bool,
}

pub struct ClaudeStats {
    pub total_sessions: usize,
    pub today: usize,
    pub this_week: usize,
}

pub enum MetricsMsg {
    System(SystemMetrics),
    Dashboard(DashboardMetrics),
    Claude(ClaudeStats),
}
```

- [ ] **Step 3: Add mod system to main.rs**

Add `mod system;` after the existing mod declarations in `src/main.rs` (after line 9, the `mod ui;` line).

- [ ] **Step 4: Verify it compiles**

Run: `cd /Users/andy/dev/dev-env && cargo check 2>&1`
Expected: compiles (warnings about unused are fine)

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/system.rs src/main.rs
git commit -m "feat: add sysinfo dependency and system metric types"
```

---

### Task 2: Implement parsing functions with tests

All parsing functions go in `src/system.rs`. Each takes raw command output as `&str` and returns parsed data. This separation makes them unit-testable.

**Files:**
- Modify: `src/system.rs`

- [ ] **Step 1: Write parsing function tests**

Add to the bottom of `src/system.rs`:

```rust
pub fn parse_battery_output(output: &str) -> (Option<f32>, bool) {
    for line in output.lines() {
        if let Some(pct_pos) = line.find('%') {
            let before = &line[..pct_pos];
            let start = before
                .rfind(|c: char| !c.is_ascii_digit())
                .map(|i| i + 1)
                .unwrap_or(0);
            if let Ok(pct) = before[start..].parse::<f32>() {
                let charging = line.contains("charging")
                    && !line.contains("not charging")
                    && !line.contains("discharging");
                return (Some(pct), charging);
            }
        }
    }
    (None, false)
}

pub fn parse_wifi_output(output: &str) -> (Option<String>, Option<i32>) {
    let mut ssid = None;
    let mut rssi = None;
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("SSID:") {
            ssid = Some(trimmed.trim_start_matches("SSID:").trim().to_string());
        } else if trimmed.starts_with("agrCtlRSSI:") {
            rssi = trimmed
                .trim_start_matches("agrCtlRSSI:")
                .trim()
                .parse::<i32>()
                .ok();
        }
    }
    (ssid, rssi)
}

pub fn parse_ports_output(output: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }
        let process = parts[0].to_string();
        let addr = parts[8];
        if let Some(colon_pos) = addr.rfind(':') {
            if let Ok(port) = addr[colon_pos + 1..].parse::<u16>() {
                if !ports.iter().any(|p: &PortInfo| p.port == port) {
                    ports.push(PortInfo { port, process });
                }
            }
        }
    }
    ports.sort_by_key(|p| p.port);
    ports
}

pub fn parse_docker_output(output: &str) -> (usize, usize) {
    let mut running = 0;
    let mut stopped = 0;
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains("Up") {
            running += 1;
        } else {
            stopped += 1;
        }
    }
    (running, stopped)
}

pub fn parse_tmux_output(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            Some(trimmed.split(':').next().unwrap_or(trimmed).to_string())
        })
        .collect()
}

pub fn parse_bluetooth_output(output: &str) -> Vec<BtDevice> {
    let mut devices = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_connected = false;

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Name:") {
            if let Some(name) = current_name.take() {
                devices.push(BtDevice {
                    name,
                    connected: current_connected,
                });
            }
            current_name = Some(trimmed.trim_start_matches("Name:").trim().to_string());
            current_connected = false;
        } else if trimmed.starts_with("Connected:") {
            let val = trimmed.trim_start_matches("Connected:").trim().to_lowercase();
            current_connected = val == "yes" || val == "connected";
        }
    }
    if let Some(name) = current_name {
        devices.push(BtDevice {
            name,
            connected: current_connected,
        });
    }
    devices
}

pub fn parse_iostat_output(output: &str) -> (Option<f64>, Option<f64>) {
    let lines: Vec<&str> = output.lines().collect();
    for line in lines.iter().rev() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let read = parts[parts.len() - 2].parse::<f64>().ok();
            let write = parts[parts.len() - 1].parse::<f64>().ok();
            if read.is_some() && write.is_some() {
                return (read, write);
            }
        }
    }
    (None, None)
}

pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn wifi_signal_label(rssi: i32) -> &'static str {
    if rssi > -50 {
        "Excellent"
    } else if rssi > -60 {
        "Good"
    } else if rssi > -70 {
        "Fair"
    } else {
        "Weak"
    }
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_bytes_per_sec(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB/s", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB/s", bytes as f64 / 1024.0)
    } else {
        format!("{} B/s", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_battery_charging() {
        let output = r#"Now drawing from 'AC Power'
 -InternalBattery-0 (id=12345678)	87%; charging; 1:23 remaining present: true"#;
        let (pct, charging) = parse_battery_output(output);
        assert_eq!(pct, Some(87.0));
        assert!(charging);
    }

    #[test]
    fn parse_battery_discharging() {
        let output = r#"Now drawing from 'Battery Power'
 -InternalBattery-0 (id=12345678)	62%; discharging; 3:45 remaining present: true"#;
        let (pct, charging) = parse_battery_output(output);
        assert_eq!(pct, Some(62.0));
        assert!(!charging);
    }

    #[test]
    fn parse_battery_not_charging() {
        let output = r#"Now drawing from 'AC Power'
 -InternalBattery-0 (id=12345678)	100%; not charging present: true"#;
        let (pct, charging) = parse_battery_output(output);
        assert_eq!(pct, Some(100.0));
        assert!(!charging);
    }

    #[test]
    fn parse_battery_no_battery() {
        let output = "No batteries available.\n";
        let (pct, charging) = parse_battery_output(output);
        assert_eq!(pct, None);
        assert!(!charging);
    }

    #[test]
    fn parse_wifi_connected() {
        let output = r#"     agrCtlRSSI: -52
     agrExtRSSI: 0
    agrCtlNoise: -88
    agrExtNoise: 0
          state: running
        op mode: station
     lastTxRate: 866
        maxRate: 72
lastAssocStatus: 0
    802.11 auth: open
      link auth: wpa2-psk
          BSSID: aa:bb:cc:dd:ee:ff
           SSID: MyHomeNetwork
            MCS: 9
  guardInterval: 800
            NSS: 2
        channel: 149,80"#;
        let (ssid, rssi) = parse_wifi_output(output);
        assert_eq!(ssid, Some("MyHomeNetwork".to_string()));
        assert_eq!(rssi, Some(-52));
    }

    #[test]
    fn parse_wifi_disconnected() {
        let output = "AirPort: Off\n";
        let (ssid, rssi) = parse_wifi_output(output);
        assert_eq!(ssid, None);
        assert_eq!(rssi, None);
    }

    #[test]
    fn parse_ports_basic() {
        let output = "node      1234 andy   23u  IPv4 0x1234  0t0  TCP *:3000 (LISTEN)
postgres  5678 andy   12u  IPv4 0x5678  0t0  TCP 127.0.0.1:5432 (LISTEN)
node      1234 andy   24u  IPv6 0x9abc  0t0  TCP *:3000 (LISTEN)";
        let ports = parse_ports_output(output);
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].port, 3000);
        assert_eq!(ports[0].process, "node");
        assert_eq!(ports[1].port, 5432);
        assert_eq!(ports[1].process, "postgres");
    }

    #[test]
    fn parse_docker_mixed() {
        let output = "abc123  myapp  Up 2 hours
def456  redis  Up 5 minutes
ghi789  nginx  Exited (0) 3 hours ago";
        let (running, stopped) = parse_docker_output(output);
        assert_eq!(running, 2);
        assert_eq!(stopped, 1);
    }

    #[test]
    fn parse_docker_empty() {
        let (running, stopped) = parse_docker_output("");
        assert_eq!(running, 0);
        assert_eq!(stopped, 0);
    }

    #[test]
    fn parse_tmux_sessions() {
        let output = "dev: 3 windows (created Mon May  1 10:00:00 2026)
ops: 1 windows (created Mon May  1 09:00:00 2026)";
        let sessions = parse_tmux_output(output);
        assert_eq!(sessions, vec!["dev", "ops"]);
    }

    #[test]
    fn parse_tmux_empty() {
        let sessions = parse_tmux_output("");
        assert!(sessions.is_empty());
    }

    #[test]
    fn parse_bluetooth_devices() {
        let output = r#"      Name: AirPods Pro
      Connected: Yes
      Name: Magic Keyboard
      Connected: Yes
      Name: Old Speaker
      Connected: No"#;
        let devices = parse_bluetooth_output(output);
        assert_eq!(devices.len(), 3);
        assert_eq!(devices[0].name, "AirPods Pro");
        assert!(devices[0].connected);
        assert_eq!(devices[1].name, "Magic Keyboard");
        assert!(devices[1].connected);
        assert_eq!(devices[2].name, "Old Speaker");
        assert!(!devices[2].connected);
    }

    #[test]
    fn format_uptime_days() {
        assert_eq!(format_uptime(90000), "1d 1h 0m");
    }

    #[test]
    fn format_uptime_hours() {
        assert_eq!(format_uptime(7380), "2h 3m");
    }

    #[test]
    fn format_uptime_minutes() {
        assert_eq!(format_uptime(300), "5m");
    }

    #[test]
    fn wifi_signal_levels() {
        assert_eq!(wifi_signal_label(-40), "Excellent");
        assert_eq!(wifi_signal_label(-55), "Good");
        assert_eq!(wifi_signal_label(-65), "Fair");
        assert_eq!(wifi_signal_label(-80), "Weak");
    }

    #[test]
    fn format_bytes_scales() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(5_242_880), "5.0 MB");
        assert_eq!(format_bytes(2_147_483_648), "2.0 GB");
    }

    #[test]
    fn format_bytes_per_sec_scales() {
        assert_eq!(format_bytes_per_sec(500), "500 B/s");
        assert_eq!(format_bytes_per_sec(1_048_576), "1.0 MB/s");
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cd /Users/andy/dev/dev-env && cargo test -- system 2>&1`
Expected: all tests pass

- [ ] **Step 3: Commit**

```bash
git add src/system.rs
git commit -m "feat: add system metric parsing functions with tests"
```

---

### Task 3: Implement collection functions and background collector

**Files:**
- Modify: `src/system.rs`

- [ ] **Step 1: Add collection functions**

Add these functions to `src/system.rs`, above the `#[cfg(test)]` section:

```rust
fn collect_battery() -> (Option<f32>, bool) {
    std::process::Command::new("pmset")
        .args(["-g", "batt"])
        .output()
        .ok()
        .map(|out| parse_battery_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or((None, false))
}

fn collect_wifi() -> (Option<String>, Option<i32>) {
    std::process::Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
        .arg("-I")
        .output()
        .ok()
        .map(|out| parse_wifi_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or((None, None))
}

fn collect_ports() -> Vec<PortInfo> {
    std::process::Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n"])
        .output()
        .ok()
        .map(|out| parse_ports_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or_default()
}

fn collect_docker() -> (usize, usize, bool) {
    match std::process::Command::new("docker")
        .args(["ps", "-a", "--format", "{{.ID}}  {{.Names}}  {{.Status}}"])
        .output()
    {
        Ok(out) if out.status.success() => {
            let (r, s) = parse_docker_output(&String::from_utf8_lossy(&out.stdout));
            (r, s, true)
        }
        _ => (0, 0, false),
    }
}

fn collect_tmux() -> Vec<String> {
    std::process::Command::new("tmux")
        .args(["list-sessions"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| parse_tmux_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or_default()
}

fn collect_bluetooth() -> Vec<BtDevice> {
    std::process::Command::new("system_profiler")
        .args(["SPBluetoothDataType"])
        .output()
        .ok()
        .map(|out| parse_bluetooth_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or_default()
}

fn collect_iostat() -> (Option<f64>, Option<f64>) {
    std::process::Command::new("iostat")
        .args(["-d", "-c", "2", "-w", "1"])
        .output()
        .ok()
        .map(|out| parse_iostat_output(&String::from_utf8_lossy(&out.stdout)))
        .unwrap_or((None, None))
}

fn collect_brew_counts() -> (usize, usize) {
    let formulae = std::process::Command::new("brew")
        .args(["list", "--formula"])
        .output()
        .ok()
        .map(|out| {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count()
        })
        .unwrap_or(0);

    let casks = std::process::Command::new("brew")
        .args(["list", "--cask"])
        .output()
        .ok()
        .map(|out| {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count()
        })
        .unwrap_or(0);

    (formulae, casks)
}

fn collect_claude_stats() -> ClaudeStats {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/Users/andy".to_string());
    let projects_dir = PathBuf::from(home).join(".claude/projects");

    let mut total = 0usize;
    let mut today = 0usize;
    let mut this_week = 0usize;

    let now = std::time::SystemTime::now();
    let one_day = Duration::from_secs(24 * 60 * 60);
    let one_week = Duration::from_secs(7 * 24 * 60 * 60);

    fn walk_jsonl(
        dir: &PathBuf,
        total: &mut usize,
        today: &mut usize,
        this_week: &mut usize,
        now: std::time::SystemTime,
        one_day: Duration,
        one_week: Duration,
    ) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_jsonl(&path, total, today, this_week, now, one_day, one_week);
            } else if path.extension().is_some_and(|e| e == "jsonl") {
                *total += 1;
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age < one_day {
                                *today += 1;
                            }
                            if age < one_week {
                                *this_week += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    walk_jsonl(
        &projects_dir,
        &mut total,
        &mut today,
        &mut this_week,
        now,
        one_day,
        one_week,
    );

    ClaudeStats {
        total_sessions: total,
        today,
        this_week,
    }
}
```

- [ ] **Step 2: Add the background collector functions**

Add these public functions to `src/system.rs`:

```rust
pub fn start_system_collector() -> mpsc::Receiver<MetricsMsg> {
    let (tx, rx) = mpsc::channel();

    let tx_claude = tx.clone();
    thread::spawn(move || {
        let stats = collect_claude_stats();
        let _ = tx_claude.send(MetricsMsg::Claude(stats));
    });

    thread::spawn(move || {
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        thread::sleep(Duration::from_millis(500));

        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let disks = Disks::new_with_refreshed_list();
            let (dt, da) = disks
                .list()
                .iter()
                .find(|d| d.mount_point() == std::path::Path::new("/"))
                .map(|d| (d.total_space(), d.available_space()))
                .unwrap_or((0, 0));

            let (battery_pct, battery_charging) = collect_battery();

            let metrics = SystemMetrics {
                cpu_usage: sys.global_cpu_usage(),
                mem_total_bytes: sys.total_memory(),
                mem_used_bytes: sys.used_memory(),
                swap_total_bytes: sys.total_swap(),
                swap_used_bytes: sys.used_swap(),
                disk_total_bytes: dt,
                disk_used_bytes: dt.saturating_sub(da),
                uptime_secs: System::uptime(),
                battery_percent: battery_pct,
                battery_charging,
            };

            if tx.send(MetricsMsg::System(metrics)).is_err() {
                break;
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    rx
}

pub fn start_dashboard_collector() -> mpsc::Receiver<MetricsMsg> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut networks = Networks::new_with_refreshed_list();
        thread::sleep(Duration::from_secs(1));

        loop {
            networks.refresh();
            let (net_down, net_up) = networks
                .list()
                .iter()
                .filter(|(name, _)| name.starts_with("en"))
                .fold((0u64, 0u64), |(down, up), (_, data)| {
                    (down + data.received(), up + data.transmitted())
                });

            let load = System::load_average();

            let components = Components::new_with_refreshed_list();
            let cpu_temp = components
                .list()
                .iter()
                .find(|c| {
                    let label = c.label().to_lowercase();
                    label.contains("cpu") || label.contains("die") || label.contains("package")
                })
                .map(|c| c.temperature());

            let (wifi_ssid, wifi_signal) = collect_wifi();
            let (disk_read, disk_write) = collect_iostat();
            let ports = collect_ports();
            let (docker_running, docker_stopped, docker_available) = collect_docker();
            let tmux_sessions = collect_tmux();
            let bluetooth_devices = collect_bluetooth();
            let (brew_formulae, brew_casks) = collect_brew_counts();

            let metrics = DashboardMetrics {
                cpu_temp,
                load_avg: [load.one, load.five, load.fifteen],
                wifi_ssid,
                wifi_signal_dbm: wifi_signal,
                net_up_bytes_sec: net_up,
                net_down_bytes_sec: net_down,
                disk_read_mb_sec: disk_read,
                disk_write_mb_sec: disk_write,
                listening_ports: ports,
                docker_running,
                docker_stopped,
                docker_available,
                tmux_sessions,
                bluetooth_devices,
                brew_formulae,
                brew_casks,
            };

            if tx.send(MetricsMsg::Dashboard(metrics)).is_err() {
                break;
            }

            thread::sleep(Duration::from_secs(5));
        }
    });

    rx
}
```

- [ ] **Step 3: Verify it compiles**

Run: `cd /Users/andy/dev/dev-env && cargo check 2>&1`
Expected: compiles with possible warnings about unused functions (that's fine, they'll be used in the next task)

- [ ] **Step 4: Run tests to make sure nothing broke**

Run: `cd /Users/andy/dev/dev-env && cargo test 2>&1`
Expected: all tests pass

- [ ] **Step 5: Commit**

```bash
git add src/system.rs
git commit -m "feat: add system metrics collection and background collectors"
```

---

### Task 4: Wire metrics into App state

**Files:**
- Modify: `src/items.rs`
- Modify: `src/app.rs`

- [ ] **Step 1: Add Dashboard to ItemId and menu items**

In `src/items.rs`, add `Dashboard` to the `ItemId` enum (after `HomebrewSync`):

```rust
Dashboard,
```

In `MenuItem::all()`, add a new menu item at the end of the vec (after Homebrew Sync):

```rust
MenuItem {
    label: "System Dashboard",
    description: "system metrics and services",
    kind: ItemKind::Action,
    status: SyncStatus::Synced,
    id: ItemId::Dashboard,
},
```

In the `setup_command` match, add `Dashboard` to the `None` arm:

```rust
ItemId::CorneFlash | ItemId::KeyboardLayout | ItemId::HomebrewSync | ItemId::Dashboard => return None,
```

In the `check_item_status` match, add `Dashboard` to the existing `Synced` arm:

```rust
ItemId::Homebrew | ItemId::CorneFlash | ItemId::KeyboardLayout | ItemId::HomebrewSync | ItemId::Dashboard => {
    SyncStatus::Synced
}
```

- [ ] **Step 2: Add AppState::Dashboard and new fields to App**

In `src/app.rs`, add imports at the top (after existing use statements):

```rust
use crate::system::{self, ClaudeStats, DashboardMetrics, MetricsMsg, SystemMetrics};
```

Add `Dashboard` variant to `AppState`:

```rust
pub enum AppState {
    Main,
    Running(usize),
    HomebrewSync(BrewSyncState),
    KeyboardLayout(usize),
    Dashboard,
    Error(String),
}
```

Add new fields to the `App` struct (after `search_query`):

```rust
pub system_metrics: Option<SystemMetrics>,
pub dashboard_metrics: Option<DashboardMetrics>,
pub claude_stats: Option<ClaudeStats>,
pub cpu_history: Vec<f64>,
metrics_rx: Option<mpsc::Receiver<MetricsMsg>>,
dashboard_rx: Option<mpsc::Receiver<MetricsMsg>>,
```

- [ ] **Step 3: Initialize new fields in App::new()**

In `App::new()`, add the new fields to the initializer (after `search_query: None`):

```rust
system_metrics: None,
dashboard_metrics: None,
claude_stats: None,
cpu_history: Vec::new(),
metrics_rx: None,
dashboard_rx: None,
```

Then after `app.start_status_checks();` add:

```rust
app.metrics_rx = Some(system::start_system_collector());
```

- [ ] **Step 4: Add metrics polling to tick()**

Add this block to the end of the `tick()` method (before the brew sync polling block):

```rust
if let Some(rx) = &self.metrics_rx {
    loop {
        match rx.try_recv() {
            Ok(MetricsMsg::System(m)) => {
                self.cpu_history.push(m.cpu_usage as f64);
                if self.cpu_history.len() > 40 {
                    self.cpu_history.remove(0);
                }
                self.system_metrics = Some(m);
            }
            Ok(MetricsMsg::Claude(s)) => {
                self.claude_stats = Some(s);
            }
            Ok(MetricsMsg::Dashboard(_)) => {}
            Err(_) => break,
        }
    }
}

if let Some(rx) = &self.dashboard_rx {
    loop {
        match rx.try_recv() {
            Ok(MetricsMsg::Dashboard(m)) => {
                self.dashboard_metrics = Some(m);
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }
}
```

- [ ] **Step 5: Add Dashboard state handling to handle_key()**

In the `Action::Confirm` match arm for `AppState::Main`, add a new arm before the existing `ItemId::CorneFlash` arm:

```rust
ItemId::Dashboard => {
    self.dashboard_rx = Some(system::start_dashboard_collector());
    self.state = AppState::Dashboard;
}
```

In the `Action::Back` match, add a new arm for Dashboard (before the `_ =>` arm):

```rust
AppState::Dashboard => {
    self.dashboard_rx = None;
    self.dashboard_metrics = None;
    self.enter_main();
}
```

Also add `AppState::Dashboard` to the `Action::ScrollDown` and `Action::ScrollUp` match arms in the `_ => {}` arms (no special behavior needed, the existing `_ => {}` will catch it).

- [ ] **Step 6: Update test_app() helper**

In `test_app()` in the tests module, add the new fields (after `search_query: None`):

```rust
system_metrics: None,
dashboard_metrics: None,
claude_stats: None,
cpu_history: Vec::new(),
metrics_rx: None,
dashboard_rx: None,
```

Also update the `items_have_correct_count` test to expect 11 items instead of 10:

```rust
assert_eq!(app.items.len(), 11);
```

And update the `all_items_has_correct_structure` test in `src/items.rs` to expect 11 items with 3 sync remaining at 7 and actions at 4:

```rust
assert_eq!(items.len(), 11);
// ...
assert_eq!(action_count, 4);
```

- [ ] **Step 7: Verify it compiles and tests pass**

Run: `cd /Users/andy/dev/dev-env && cargo test 2>&1`
Expected: all tests pass (there will be warnings about unused imports/fields in ui.rs since we haven't updated rendering yet)

- [ ] **Step 8: Commit**

```bash
git add src/items.rs src/app.rs
git commit -m "feat: wire system metrics into app state with dashboard state"
```

---

### Task 5: Implement compact panel UI on main view

**Files:**
- Modify: `src/ui.rs`

- [ ] **Step 1: Add imports**

Add to the top of `src/ui.rs`:

```rust
use crate::system;
```

- [ ] **Step 2: Add gauge and sparkline helper functions**

Add these functions before `render_error` in `src/ui.rs`:

```rust
fn gauge_color(pct: f32) -> Color {
    if pct < 50.0 {
        Color::Green
    } else if pct < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn gauge_line(label: &str, pct: f32, detail: Option<&str>) -> Line<'static> {
    let bar_width = 12usize;
    let filled = ((pct / 100.0) * bar_width as f32).round() as usize;
    let filled = filled.min(bar_width);
    let empty = bar_width - filled;
    let color = gauge_color(pct);

    let mut spans = vec![
        Span::styled(format!("  {:<5}", label), Style::default().fg(Color::DarkGray)),
        Span::styled("\u{2588}".repeat(filled), Style::default().fg(color)),
        Span::styled("\u{2591}".repeat(empty), Style::default().fg(Color::DarkGray)),
        Span::styled(format!(" {:>3.0}%", pct), Style::default().fg(Color::White)),
    ];

    if let Some(d) = detail {
        spans.push(Span::styled(format!("  {d}"), Style::default().fg(Color::DarkGray)));
    }

    Line::from(spans)
}

const SPARK_CHARS: &[char] = &[' ', '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}'];

fn cpu_spark_line(history: &[f64]) -> Line<'static> {
    let mut spans = vec![Span::styled("  ", Style::default())];
    for &val in history {
        let idx = ((val / 100.0) * 8.0).round() as usize;
        let ch = SPARK_CHARS[idx.min(8)];
        let color = gauge_color(val as f32);
        spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
    }
    spans.push(Span::styled("  cpu", Style::default().fg(Color::DarkGray)));
    Line::from(spans)
}
```

- [ ] **Step 3: Refactor render_main into two functions**

Rename the existing `render_main` function to `render_menu_list`. Then add a new `render_main` that handles the split layout:

```rust
fn render_main(f: &mut Frame, app: &App, area: Rect) {
    let show_panel = area.width >= 70;

    if show_panel {
        let chunks = Layout::horizontal([
            Constraint::Min(35),
            Constraint::Length(30),
        ])
        .split(area);

        render_menu_list(f, app, chunks[0]);
        render_compact_panel(f, app, chunks[1]);
    } else {
        render_menu_list(f, app, area);
    }
}
```

The existing `render_main` function becomes `render_menu_list` with the same body unchanged.

- [ ] **Step 4: Add render_compact_panel function**

```rust
fn render_compact_panel(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  System",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("  {}", "\u{2500}".repeat(22)),
        Style::default().fg(Color::DarkGray),
    )));

    if let Some(m) = &app.system_metrics {
        lines.push(gauge_line("CPU", m.cpu_usage, None));

        let mem_pct = if m.mem_total_bytes > 0 {
            (m.mem_used_bytes as f32 / m.mem_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        lines.push(gauge_line("MEM", mem_pct, None));

        let disk_pct = if m.disk_total_bytes > 0 {
            (m.disk_used_bytes as f32 / m.disk_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        lines.push(gauge_line("DSK", disk_pct, None));

        let swap_pct = if m.swap_total_bytes > 0 {
            (m.swap_used_bytes as f32 / m.swap_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        lines.push(gauge_line("SWP", swap_pct, None));

        lines.push(Line::from(""));

        if !app.cpu_history.is_empty() {
            lines.push(cpu_spark_line(&app.cpu_history));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![
            Span::styled("  Uptime  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                system::format_uptime(m.uptime_secs),
                Style::default().fg(Color::White),
            ),
        ]));

        if let Some(pct) = m.battery_percent {
            let status = if m.battery_charging {
                " charging"
            } else {
                ""
            };
            let color = if pct > 50.0 {
                Color::Green
            } else if pct > 20.0 {
                Color::Yellow
            } else {
                Color::Red
            };
            lines.push(Line::from(vec![
                Span::styled("  Battery ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.0}%", pct), Style::default().fg(color)),
                Span::styled(
                    status.to_string(),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            format!("  {} loading...", spinner(app.spinner_tick)),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Claude Code",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("  {}", "\u{2500}".repeat(22)),
        Style::default().fg(Color::DarkGray),
    )));

    if let Some(s) = &app.claude_stats {
        lines.push(Line::from(vec![
            Span::styled("  Sessions  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                s.total_sessions.to_string(),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Today     ", Style::default().fg(Color::DarkGray)),
            Span::styled(s.today.to_string(), Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  This week ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                s.this_week.to_string(),
                Style::default().fg(Color::White),
            ),
        ]));
    } else {
        lines.push(Line::from(Span::styled(
            format!("  {} loading...", spinner(app.spinner_tick)),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}
```

- [ ] **Step 5: Update render_content to handle Dashboard state**

In `render_content`, add the `Dashboard` arm:

```rust
fn render_content(f: &mut Frame, app: &App, area: Rect) {
    match &app.state {
        AppState::Main => render_main(f, app, area),
        AppState::Running(_) => render_running(f, app, area),
        AppState::HomebrewSync(sub) => render_brew_sync(f, app, area, sub),
        AppState::KeyboardLayout(layer) => render_keyboard_layout(f, app, area, *layer),
        AppState::Dashboard => render_dashboard(f, app, area),
        AppState::Error(msg) => render_error(f, area, msg),
    }
}
```

Add a placeholder `render_dashboard` (will be implemented in Task 6):

```rust
fn render_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let lines = vec![Line::from(Span::styled(
        format!("  {} Loading dashboard...", spinner(app.spinner_tick)),
        Style::default().fg(Color::Yellow),
    ))];
    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}
```

- [ ] **Step 6: Update header and footer for Dashboard state**

In `render_header`, add to the `title_right` match:

```rust
AppState::Dashboard => " System Dashboard ".to_string(),
```

In `render_footer`, add the Dashboard hints:

```rust
AppState::Dashboard => {
    vec![("Esc", "back"), (":q", "quit")]
}
```

- [ ] **Step 7: Verify it compiles and tests pass**

Run: `cd /Users/andy/dev/dev-env && cargo test 2>&1`
Expected: all tests pass

- [ ] **Step 8: Build and manually test**

Run: `cd /Users/andy/dev/dev-env && cargo build --release 2>&1`
Expected: compiles successfully

- [ ] **Step 9: Commit**

```bash
git add src/ui.rs
git commit -m "feat: add compact system panel to main view"
```

---

### Task 6: Implement full dashboard UI

**Files:**
- Modify: `src/ui.rs`

- [ ] **Step 1: Replace the placeholder render_dashboard with the full implementation**

Replace the placeholder `render_dashboard` function with:

```rust
fn render_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let columns = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(area);

    render_dashboard_left(f, app, columns[0]);
    render_dashboard_right(f, app, columns[1]);
}

fn render_dashboard_left(f: &mut Frame, app: &App, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Resources",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    let sep_width = (area.width as usize).saturating_sub(4);
    lines.push(Line::from(Span::styled(
        format!("  {}", "\u{2500}".repeat(sep_width)),
        Style::default().fg(Color::DarkGray),
    )));

    if let Some(m) = &app.system_metrics {
        let temp_str = app
            .dashboard_metrics
            .as_ref()
            .and_then(|d| d.cpu_temp)
            .map(|t| format!("{:.0}\u{00B0}C", t));

        lines.push(gauge_line("CPU", m.cpu_usage, temp_str.as_deref()));

        let mem_pct = if m.mem_total_bytes > 0 {
            (m.mem_used_bytes as f32 / m.mem_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        let mem_detail = format!(
            "{} / {}",
            system::format_bytes(m.mem_used_bytes),
            system::format_bytes(m.mem_total_bytes)
        );
        lines.push(gauge_line("MEM", mem_pct, Some(&mem_detail)));

        let disk_pct = if m.disk_total_bytes > 0 {
            (m.disk_used_bytes as f32 / m.disk_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        let disk_detail = format!(
            "{} / {}",
            system::format_bytes(m.disk_used_bytes),
            system::format_bytes(m.disk_total_bytes)
        );
        lines.push(gauge_line("DSK", disk_pct, Some(&disk_detail)));

        let swap_pct = if m.swap_total_bytes > 0 {
            (m.swap_used_bytes as f32 / m.swap_total_bytes as f32) * 100.0
        } else {
            0.0
        };
        let swap_detail = format!(
            "{} / {}",
            system::format_bytes(m.swap_used_bytes),
            system::format_bytes(m.swap_total_bytes)
        );
        lines.push(gauge_line("SWP", swap_pct, Some(&swap_detail)));

        if let Some(d) = &app.dashboard_metrics {
            lines.push(Line::from(vec![
                Span::styled("  LOAD  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{:.2}  {:.2}  {:.2}", d.load_avg[0], d.load_avg[1], d.load_avg[2]),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        lines.push(Line::from(""));

        if !app.cpu_history.is_empty() {
            lines.push(cpu_spark_line(&app.cpu_history));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "  System",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!("  {}", "\u{2500}".repeat(sep_width)),
            Style::default().fg(Color::DarkGray),
        )));

        lines.push(Line::from(vec![
            Span::styled("  Uptime   ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                system::format_uptime(m.uptime_secs),
                Style::default().fg(Color::White),
            ),
        ]));

        if let Some(pct) = m.battery_percent {
            let status = if m.battery_charging {
                " charging"
            } else {
                ""
            };
            let color = if pct > 50.0 {
                Color::Green
            } else if pct > 20.0 {
                Color::Yellow
            } else {
                Color::Red
            };
            lines.push(Line::from(vec![
                Span::styled("  Battery  ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.0}%", pct), Style::default().fg(color)),
                Span::styled(status.to_string(), Style::default().fg(Color::DarkGray)),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Claude Code",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!("  {}", "\u{2500}".repeat(sep_width)),
            Style::default().fg(Color::DarkGray),
        )));

        if let Some(s) = &app.claude_stats {
            lines.push(Line::from(vec![
                Span::styled("  Sessions   ", Style::default().fg(Color::DarkGray)),
                Span::styled(s.total_sessions.to_string(), Style::default().fg(Color::White)),
                Span::styled("    Today  ", Style::default().fg(Color::DarkGray)),
                Span::styled(s.today.to_string(), Style::default().fg(Color::White)),
                Span::styled("    This week  ", Style::default().fg(Color::DarkGray)),
                Span::styled(s.this_week.to_string(), Style::default().fg(Color::White)),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            format!("  {} loading...", spinner(app.spinner_tick)),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_dashboard_right(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();
    let sep_width = (inner.width as usize).saturating_sub(4);

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Network & I/O",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        format!("  {}", "\u{2500}".repeat(sep_width)),
        Style::default().fg(Color::DarkGray),
    )));

    if let Some(d) = &app.dashboard_metrics {
        if let Some(ref ssid) = d.wifi_ssid {
            let signal_label = d
                .wifi_signal_dbm
                .map(system::wifi_signal_label)
                .unwrap_or("--");
            let signal_color = match signal_label {
                "Excellent" | "Good" => Color::Green,
                "Fair" => Color::Yellow,
                _ => Color::Red,
            };
            lines.push(Line::from(vec![
                Span::styled("  Wi-Fi   ", Style::default().fg(Color::DarkGray)),
                Span::styled(ssid.clone(), Style::default().fg(Color::White)),
                Span::styled(" (", Style::default().fg(Color::DarkGray)),
                Span::styled(signal_label.to_string(), Style::default().fg(signal_color)),
                Span::styled(")", Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("  Wi-Fi   ", Style::default().fg(Color::DarkGray)),
                Span::styled("disconnected", Style::default().fg(Color::Red)),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("  Net     ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "\u{2191} ",
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                system::format_bytes_per_sec(d.net_up_bytes_sec),
                Style::default().fg(Color::White),
            ),
            Span::styled("  ", Style::default()),
            Span::styled(
                "\u{2193} ",
                Style::default().fg(Color::Cyan),
            ),
            Span::styled(
                system::format_bytes_per_sec(d.net_down_bytes_sec),
                Style::default().fg(Color::White),
            ),
        ]));

        if d.disk_read_mb_sec.is_some() || d.disk_write_mb_sec.is_some() {
            lines.push(Line::from(vec![
                Span::styled("  DSK I/O ", Style::default().fg(Color::DarkGray)),
                Span::styled("R: ", Style::default().fg(Color::Green)),
                Span::styled(
                    d.disk_read_mb_sec
                        .map(|v| format!("{:.1} MB/s", v))
                        .unwrap_or_else(|| "n/a".to_string()),
                    Style::default().fg(Color::White),
                ),
                Span::styled("  W: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    d.disk_write_mb_sec
                        .map(|v| format!("{:.1} MB/s", v))
                        .unwrap_or_else(|| "n/a".to_string()),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Bluetooth",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!("  {}", "\u{2500}".repeat(sep_width)),
            Style::default().fg(Color::DarkGray),
        )));

        if d.bluetooth_devices.is_empty() {
            lines.push(Line::from(Span::styled(
                "  no devices",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for dev in &d.bluetooth_devices {
                let (status, color) = if dev.connected {
                    ("connected", Color::Green)
                } else {
                    ("disconnected", Color::DarkGray)
                };
                lines.push(Line::from(vec![
                    Span::styled(format!("  {:<20}", dev.name), Style::default().fg(Color::White)),
                    Span::styled(status.to_string(), Style::default().fg(color)),
                ]));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Services",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            format!("  {}", "\u{2500}".repeat(sep_width)),
            Style::default().fg(Color::DarkGray),
        )));

        if !d.tmux_sessions.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  tmux    ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!(
                        "{} session{} ({})",
                        d.tmux_sessions.len(),
                        if d.tmux_sessions.len() == 1 { "" } else { "s" },
                        d.tmux_sessions.join(", ")
                    ),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        if !d.listening_ports.is_empty() {
            let port_strs: Vec<String> = d
                .listening_ports
                .iter()
                .take(6)
                .map(|p| format!(":{} {}", p.port, p.process))
                .collect();
            lines.push(Line::from(vec![
                Span::styled("  Ports   ", Style::default().fg(Color::DarkGray)),
                Span::styled(port_strs.join("  "), Style::default().fg(Color::White)),
            ]));
        }

        if d.docker_available {
            lines.push(Line::from(vec![
                Span::styled("  Docker  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{} running", d.docker_running),
                    Style::default().fg(Color::Green),
                ),
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("{} stopped", d.docker_stopped),
                    Style::default().fg(if d.docker_stopped > 0 {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    }),
                ),
            ]));
        }

        lines.push(Line::from(vec![
            Span::styled("  Brew    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} formulae  {} casks", d.brew_formulae, d.brew_casks),
                Style::default().fg(Color::White),
            ),
        ]));
    } else {
        lines.push(Line::from(Span::styled(
            format!("  {} loading...", spinner(app.spinner_tick)),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}
```

- [ ] **Step 2: Verify it compiles and tests pass**

Run: `cd /Users/andy/dev/dev-env && cargo test 2>&1`
Expected: all tests pass

- [ ] **Step 3: Build the release binary**

Run: `cd /Users/andy/dev/dev-env && cargo build --release 2>&1`
Expected: compiles successfully

- [ ] **Step 4: Commit**

```bash
git add src/ui.rs
git commit -m "feat: add full system dashboard view with two-column grid"
```

---

### Task 7: Fix compilation issues and integration test

This task covers any compilation errors from the sysinfo API (version differences in method names, iteration patterns) and verifying the full application works.

**Files:**
- Potentially: `src/system.rs`, `src/app.rs`, `src/ui.rs`

- [ ] **Step 1: Build and fix any compilation errors**

Run: `cd /Users/andy/dev/dev-env && cargo build --release 2>&1`

If there are sysinfo API issues, common fixes:
- If `Disks::list()` doesn't exist, try iterating with `disks.iter()` or `&disks` directly
- If `Networks::list()` doesn't exist, try `networks.iter()` or `&networks`
- If `Components::list()` doesn't exist, try `components.iter()` or `&components`
- If `System::load_average()` doesn't exist, use `sysctl -n vm.loadavg` via Command instead
- If `MINIMUM_CPU_UPDATE_INTERVAL` doesn't exist, use `Duration::from_millis(200)`

- [ ] **Step 2: Run all tests**

Run: `cd /Users/andy/dev/dev-env && cargo test 2>&1`
Expected: all tests pass

- [ ] **Step 3: Manual smoke test**

Run: `cd /Users/andy/dev/dev-env && ./target/release/os`

Verify:
1. Main view shows the compact panel on the right with CPU/MEM/DSK/SWP gauges
2. CPU sparkline appears after a few seconds
3. Uptime and battery are shown
4. Claude Code session counts are shown
5. Navigate to "System Dashboard" and press Enter
6. Full dashboard shows two columns with all metrics
7. Press Esc to go back to main view
8. Press :q to quit

- [ ] **Step 4: Commit any fixes**

```bash
git add -A
git commit -m "fix: resolve compilation issues and verify dashboard"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Add sysinfo dep + types | Cargo.toml, system.rs, main.rs |
| 2 | Parsing functions + tests | system.rs |
| 3 | Collection + background collectors | system.rs |
| 4 | Wire into App state | items.rs, app.rs |
| 5 | Compact panel UI | ui.rs |
| 6 | Full dashboard UI | ui.rs |
| 7 | Fix compilation + integration test | any |
