use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RunInfo {
    #[serde(rename = "databaseId")]
    pub id: u64,
    #[serde(rename = "headSha")]
    pub sha: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "displayTitle")]
    pub title: String,
}

const REPO: &str = "andyhmltn/dev-env";
const WORKFLOW: &str = "build-corne.yml";

pub fn fetch_latest_run() -> Result<RunInfo> {
    let output = Command::new("gh")
        .args([
            "run",
            "list",
            "--repo",
            REPO,
            "--workflow",
            WORKFLOW,
            "--status",
            "success",
            "--limit",
            "1",
            "--json",
            "databaseId,headSha,createdAt,displayTitle",
        ])
        .output()
        .context("Failed to run gh CLI -- is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("gh run list failed: {stderr}");
    }

    let runs: Vec<RunInfo> =
        serde_json::from_slice(&output.stdout).context("Failed to parse gh output")?;

    runs.into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No successful workflow runs found"))
}

pub fn download_artifacts(run_id: u64) -> Result<PathBuf> {
    let dir = tempfile::tempdir()
        .context("Failed to create temp directory")?
        .keep();

    let output = Command::new("gh")
        .args([
            "run",
            "download",
            &run_id.to_string(),
            "--repo",
            REPO,
            "--dir",
            &dir.to_string_lossy(),
        ])
        .output()
        .context("Failed to run gh run download")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("gh run download failed: {stderr}");
    }

    Ok(dir)
}

pub fn find_firmware_files(dir: &Path) -> Result<(PathBuf, PathBuf)> {
    let mut left = None;
    let mut right = None;

    for entry in collect_files(dir)? {
        let path_str = entry.to_string_lossy().to_string();
        if path_str.ends_with(".uf2") {
            if path_str.contains("corne_left") {
                left = Some(entry);
            } else if path_str.contains("corne_right") {
                right = Some(entry);
            }
        }
    }

    let left = left.ok_or_else(|| anyhow::anyhow!("Left firmware .uf2 not found in artifacts"))?;
    let right =
        right.ok_or_else(|| anyhow::anyhow!("Right firmware .uf2 not found in artifacts"))?;

    Ok((left, right))
}

fn collect_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_recursive(dir, &mut files)?;
    Ok(files)
}

fn collect_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_recursive(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_run_info() {
        let json = r#"[{
            "databaseId": 12345,
            "headSha": "abc1234def5678",
            "createdAt": "2026-04-30T14:22:00Z",
            "displayTitle": "feat: update keymap"
        }]"#;

        let runs: Vec<RunInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].id, 12345);
        assert_eq!(runs[0].sha, "abc1234def5678");
        assert_eq!(runs[0].title, "feat: update keymap");
    }

    #[test]
    fn parse_empty_runs() {
        let json = "[]";
        let runs: Vec<RunInfo> = serde_json::from_str(json).unwrap();
        assert!(runs.is_empty());
    }

    #[test]
    fn find_firmware_in_nested_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let left_dir = dir.path().join("nice_nano_v2-corne_left-zmk");
        let right_dir = dir.path().join("nice_nano_v2-corne_right-zmk");
        std::fs::create_dir_all(&left_dir).unwrap();
        std::fs::create_dir_all(&right_dir).unwrap();

        std::fs::write(left_dir.join("firmware-corne_left.uf2"), b"left").unwrap();
        std::fs::write(right_dir.join("firmware-corne_right.uf2"), b"right").unwrap();

        let (left, right) = find_firmware_files(dir.path()).unwrap();
        assert!(left.to_string_lossy().contains("corne_left"));
        assert!(right.to_string_lossy().contains("corne_right"));
    }

    #[test]
    fn find_firmware_by_directory_name() {
        let dir = tempfile::tempdir().unwrap();
        let left_dir = dir.path().join("corne_left-stuff");
        let right_dir = dir.path().join("corne_right-stuff");
        std::fs::create_dir_all(&left_dir).unwrap();
        std::fs::create_dir_all(&right_dir).unwrap();

        std::fs::write(left_dir.join("zmk.uf2"), b"left").unwrap();
        std::fs::write(right_dir.join("zmk.uf2"), b"right").unwrap();

        let (left, right) = find_firmware_files(dir.path()).unwrap();
        assert!(left.to_string_lossy().contains("corne_left"));
        assert!(right.to_string_lossy().contains("corne_right"));
    }

    #[test]
    fn missing_firmware_errors() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_firmware_files(dir.path()).is_err());
    }
}
