use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub enum PkgKind {
    Formula,
    Cask,
}

#[derive(Debug, Clone)]
pub struct UntrackedPkg {
    pub name: String,
    pub kind: PkgKind,
}

pub fn parse_install_sh(repo_root: &Path) -> Result<(Vec<String>, Vec<String>)> {
    let content =
        std::fs::read_to_string(repo_root.join("homebrew/install.sh")).context("reading install.sh")?;

    let formulae = extract_array(&content, "FORMULAE");
    let casks = extract_array(&content, "CASKS");

    Ok((formulae, casks))
}

fn extract_array(content: &str, name: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut in_array = false;
    let opener = format!("{name}=(");

    for line in content.lines() {
        if line.trim_start().starts_with(&opener) {
            in_array = true;
            continue;
        }
        if in_array {
            let trimmed = line.trim();
            if trimmed == ")" || trimmed.starts_with(')') {
                break;
            }
            let without_comment = if let Some(idx) = trimmed.find('#') {
                &trimmed[..idx]
            } else {
                trimmed
            };
            let pkg = without_comment.trim();
            if !pkg.is_empty() {
                results.push(pkg.to_string());
            }
        }
    }

    results
}

pub fn get_installed_all() -> Result<(Vec<String>, Vec<String>)> {
    let formulae_output = Command::new("brew")
        .args(["list", "--formula"])
        .output()
        .context("failed to run brew list --formula")?;

    let formulae: Vec<String> = String::from_utf8_lossy(&formulae_output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let casks_output = Command::new("brew")
        .args(["list", "--cask"])
        .output()
        .context("failed to run brew list --cask")?;

    let casks: Vec<String> = String::from_utf8_lossy(&casks_output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok((formulae, casks))
}

pub fn get_installed_leaves() -> Result<(Vec<String>, Vec<String>)> {
    let formulae_output = Command::new("brew")
        .arg("leaves")
        .output()
        .context("failed to run brew leaves")?;

    let formulae: Vec<String> = String::from_utf8_lossy(&formulae_output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    let casks_output = Command::new("brew")
        .args(["list", "--cask"])
        .output()
        .context("failed to run brew list --cask")?;

    let casks: Vec<String> = String::from_utf8_lossy(&casks_output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok((formulae, casks))
}

pub fn find_untracked(repo_root: &Path) -> Result<Vec<UntrackedPkg>> {
    let (tracked_formulae, tracked_casks) = parse_install_sh(repo_root)?;
    let (installed_formulae, installed_casks) = get_installed_leaves()?;

    let mut untracked = Vec::new();

    for f in &installed_formulae {
        if !tracked_formulae.contains(f) {
            untracked.push(UntrackedPkg {
                name: f.clone(),
                kind: PkgKind::Formula,
            });
        }
    }

    for c in &installed_casks {
        if !tracked_casks.contains(c) {
            untracked.push(UntrackedPkg {
                name: c.clone(),
                kind: PkgKind::Cask,
            });
        }
    }

    Ok(untracked)
}

pub fn check_homebrew_status(repo_root: &Path) -> Result<(usize, usize)> {
    let (tracked_formulae, tracked_casks) = parse_install_sh(repo_root)?;
    let total = tracked_formulae.len() + tracked_casks.len();
    let (installed_formulae, installed_casks) = get_installed_all()?;

    let installed_count = tracked_formulae
        .iter()
        .filter(|f| installed_formulae.contains(f))
        .count()
        + tracked_casks
            .iter()
            .filter(|c| installed_casks.contains(c))
            .count();

    Ok((installed_count, total))
}

pub fn add_to_install_sh(
    repo_root: &Path,
    pkg: &str,
    kind: &PkgKind,
    comment: Option<&str>,
) -> Result<()> {
    let path = repo_root.join("homebrew/install.sh");
    let content = std::fs::read_to_string(&path).context("reading install.sh")?;

    let array_name = match kind {
        PkgKind::Formula => "FORMULAE",
        PkgKind::Cask => "CASKS",
    };

    let opener = format!("{array_name}=(");
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut insert_idx = None;
    let mut in_array = false;

    for (i, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with(&opener) {
            in_array = true;
            continue;
        }
        if in_array && (line.trim() == ")" || line.trim().starts_with(')')) {
            insert_idx = Some(i);
            break;
        }
    }

    if let Some(idx) = insert_idx {
        let new_line = match comment {
            Some(c) => format!("    {pkg}    # {c}"),
            None => format!("    {pkg}"),
        };
        lines.insert(idx, new_line);
        let result = lines.join("\n") + "\n";
        std::fs::write(&path, result).context("writing install.sh")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_formulae() {
        let content = r#"FORMULAE=(
    fish        # Fish shell
    neovim      # Text editor
    ripgrep
)
CASKS=(
    claude-code # Claude Code CLI
)"#;

        let formulae = extract_array(content, "FORMULAE");
        assert_eq!(formulae, vec!["fish", "neovim", "ripgrep"]);

        let casks = extract_array(content, "CASKS");
        assert_eq!(casks, vec!["claude-code"]);
    }

    #[test]
    fn parse_empty_array() {
        let content = "FORMULAE=(\n)\n";
        let formulae = extract_array(content, "FORMULAE");
        assert!(formulae.is_empty());
    }

    #[test]
    fn parse_nonexistent_array() {
        let content = "FORMULAE=(\n    fish\n)\n";
        let result = extract_array(content, "CASKS");
        assert!(result.is_empty());
    }

    #[test]
    fn parse_strips_inline_comments() {
        let content = "FORMULAE=(\n    fish    # a fish shell\n    vim  # editor\n)\n";
        let result = extract_array(content, "FORMULAE");
        assert_eq!(result, vec!["fish", "vim"]);
    }

    #[test]
    fn parse_skips_blank_lines() {
        let content = "FORMULAE=(\n    fish\n\n    vim\n)\n";
        let result = extract_array(content, "FORMULAE");
        assert_eq!(result, vec!["fish", "vim"]);
    }

    #[test]
    fn add_to_install_sh_formula() {
        let dir = std::env::temp_dir().join("os_test_add_formula");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("homebrew")).unwrap();

        let content = "FORMULAE=(\n    fish\n    vim\n)\nCASKS=(\n    chrome\n)\n";
        std::fs::write(dir.join("homebrew/install.sh"), content).unwrap();

        add_to_install_sh(&dir, "wget", &PkgKind::Formula, None).unwrap();

        let result = std::fs::read_to_string(dir.join("homebrew/install.sh")).unwrap();
        let formulae = extract_array(&result, "FORMULAE");
        assert_eq!(formulae, vec!["fish", "vim", "wget"]);

        let casks = extract_array(&result, "CASKS");
        assert_eq!(casks, vec!["chrome"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn add_to_install_sh_cask_with_comment() {
        let dir = std::env::temp_dir().join("os_test_add_cask");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("homebrew")).unwrap();

        let content = "FORMULAE=(\n    fish\n)\nCASKS=(\n    chrome\n)\n";
        std::fs::write(dir.join("homebrew/install.sh"), content).unwrap();

        add_to_install_sh(&dir, "firefox", &PkgKind::Cask, Some("web browser")).unwrap();

        let result = std::fs::read_to_string(dir.join("homebrew/install.sh")).unwrap();
        assert!(result.contains("    firefox    # web browser"));

        let casks = extract_array(&result, "CASKS");
        assert_eq!(casks, vec!["chrome", "firefox"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_real_install_sh() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        if repo.join("homebrew/install.sh").exists() {
            let (formulae, casks) = parse_install_sh(repo).unwrap();
            assert!(formulae.contains(&"fish".to_string()));
            assert!(formulae.contains(&"neovim".to_string()));
            assert!(formulae.contains(&"ripgrep".to_string()));
            assert!(casks.contains(&"claude-code".to_string()));
            assert!(formulae.len() >= 20);
        }
    }
}
