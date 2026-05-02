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
    pub disk_throughput_mb_sec: Option<f64>,
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

pub fn parse_iostat_output(output: &str) -> Option<f64> {
    let lines: Vec<&str> = output.lines().collect();
    for line in lines.iter().rev() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(mb_sec) = parts[parts.len() - 1].parse::<f64>() {
                if parts[parts.len() - 2].parse::<f64>().is_ok() {
                    return Some(mb_sec);
                }
            }
        }
    }
    None
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

fn collect_iostat() -> Option<f64> {
    std::process::Command::new("iostat")
        .args(["-d", "-K", "-c", "2", "-w", "1"])
        .output()
        .ok()
        .and_then(|out| parse_iostat_output(&String::from_utf8_lossy(&out.stdout)))
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
    let Ok(home) = std::env::var("HOME") else {
        return ClaudeStats { total_sessions: 0, today: 0, this_week: 0 };
    };
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

        let mut disks = Disks::new_with_refreshed_list();
        loop {
            sys.refresh_cpu_usage();
            sys.refresh_memory();
            disks.refresh(true);

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
        let mut prev_down: u64 = 0;
        let mut prev_up: u64 = 0;
        let interval_secs: u64 = 5;
        thread::sleep(Duration::from_secs(1));

        networks.refresh(true);
        for (name, data) in networks.list() {
            if name.starts_with("en") {
                prev_down += data.total_received();
                prev_up += data.total_transmitted();
            }
        }

        loop {
            thread::sleep(Duration::from_secs(interval_secs));
            networks.refresh(true);

            let mut cur_down: u64 = 0;
            let mut cur_up: u64 = 0;
            for (name, data) in networks.list() {
                if name.starts_with("en") {
                    cur_down += data.total_received();
                    cur_up += data.total_transmitted();
                }
            }

            let net_down = (cur_down.saturating_sub(prev_down)) / interval_secs;
            let net_up = (cur_up.saturating_sub(prev_up)) / interval_secs;
            prev_down = cur_down;
            prev_up = cur_up;

            let load = System::load_average();

            let components = Components::new_with_refreshed_list();
            let cpu_temp = components
                .list()
                .iter()
                .find(|c| {
                    let label = c.label().to_lowercase();
                    label.contains("cpu") || label.contains("die") || label.contains("package")
                })
                .and_then(|c| c.temperature());

            let (wifi_ssid, wifi_signal) = collect_wifi();
            let disk_throughput = collect_iostat();
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
                disk_throughput_mb_sec: disk_throughput,
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
        }
    });

    rx
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

    #[test]
    fn parse_iostat_basic() {
        let output = "          disk0\n    KB/t  tps  MB/s\n   64.00   12  0.75\n";
        let throughput = parse_iostat_output(output);
        assert_eq!(throughput, Some(0.75));
    }

    #[test]
    fn parse_iostat_empty() {
        let throughput = parse_iostat_output("");
        assert_eq!(throughput, None);
    }
}
