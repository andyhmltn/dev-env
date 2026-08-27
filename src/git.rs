use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

#[derive(Debug)]
pub enum GitStatus {
    UpToDate,
    Behind(usize),
}

pub fn check_remote(repo_root: &Path) -> Result<GitStatus> {
    Command::new("git")
        .args(["-C", &repo_root.display().to_string(), "fetch", "origin", "--quiet"])
        .output()
        .context("failed to run git fetch")?;

    let local = Command::new("git")
        .args(["-C", &repo_root.display().to_string(), "rev-parse", "HEAD"])
        .output()
        .context("failed to get local HEAD")?;

    let remote = Command::new("git")
        .args(["-C", &repo_root.display().to_string(), "rev-parse", "@{u}"])
        .output()
        .context("failed to get upstream ref")?;

    let local_sha = String::from_utf8_lossy(&local.stdout).trim().to_string();
    let remote_sha = String::from_utf8_lossy(&remote.stdout).trim().to_string();

    if local_sha == remote_sha {
        return Ok(GitStatus::UpToDate);
    }

    let count_output = Command::new("git")
        .args([
            "-C",
            &repo_root.display().to_string(),
            "rev-list",
            "--count",
            &format!("{local_sha}..{remote_sha}"),
        ])
        .output()
        .context("failed to count commits behind")?;

    let count: usize = String::from_utf8_lossy(&count_output.stdout)
        .trim()
        .parse()
        .unwrap_or(1);

    Ok(GitStatus::Behind(count))
}

pub fn pull(repo_root: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["-C", &repo_root.display().to_string(), "pull"])
        .output()
        .context("failed to run git pull")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        anyhow::bail!(
            "git pull failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}
