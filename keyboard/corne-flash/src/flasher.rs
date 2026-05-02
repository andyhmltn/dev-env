use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Context, Result};

const VOLUME_PATH: &str = "/Volumes/NICENANO";
const MOUNT_SETTLE_MS: u64 = 1500;

pub fn check_root() -> bool {
    Command::new("id")
        .args(["-u"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim() == "0")
        .unwrap_or(false)
}

pub fn check_volume() -> Option<PathBuf> {
    let path = PathBuf::from(VOLUME_PATH);
    if path.exists() && path.is_dir() {
        Some(path)
    } else {
        None
    }
}

fn get_parent_disk(volume: &Path) -> Result<String> {
    let output = Command::new("diskutil")
        .args(["info", &volume.to_string_lossy()])
        .output()
        .context("Failed to run diskutil info")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("Part of Whole:") {
            if let Some(disk) = line.split(':').nth(1) {
                return Ok(disk.trim().to_string());
            }
        }
    }

    bail!("Could not determine parent disk for NICENANO")
}

pub fn flash_firmware(uf2: &Path, volume: &Path) -> Result<()> {
    thread::sleep(Duration::from_millis(MOUNT_SETTLE_MS));

    if !volume.exists() {
        bail!("NICENANO volume disappeared before copy could start");
    }

    let parent_disk = get_parent_disk(volume)?;

    let unmount = Command::new("diskutil")
        .args(["unmountDisk", &parent_disk])
        .output()
        .context("Failed to unmount NICENANO")?;

    if !unmount.status.success() {
        let stderr = String::from_utf8_lossy(&unmount.stderr);
        bail!("diskutil unmountDisk failed: {stderr}");
    }

    let raw_device = format!("/dev/r{parent_disk}");

    let dd = Command::new("dd")
        .arg(format!("if={}", uf2.display()))
        .arg(format!("of={raw_device}"))
        .arg("bs=4k")
        .output()
        .context("Failed to run dd")?;

    if !dd.status.success() {
        if !PathBuf::from(&raw_device).exists() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&dd.stderr);
        bail!("dd to {raw_device} failed: {stderr}");
    }

    Ok(())
}