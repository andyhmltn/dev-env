use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Synced,
    NotSynced,
    Partial(usize, usize),
    Checking,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    Sync,
    Action,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: &'static str,
    pub description: &'static str,
    pub kind: ItemKind,
    pub status: SyncStatus,
    pub id: ItemId,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemId {
    Homebrew,
    Neovim,
    Tmux,
    Fish,
    Claude,
    Ghostty,
    Aerospace,
    CorneFlash,
    KeyboardLayout,
    HomebrewSync,
}

impl MenuItem {
    pub fn all() -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "Homebrew",
                description: "install packages",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Homebrew,
            },
            MenuItem {
                label: "Neovim",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Neovim,
            },
            MenuItem {
                label: "Tmux",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Tmux,
            },
            MenuItem {
                label: "Fish",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Fish,
            },
            MenuItem {
                label: "Claude",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Claude,
            },
            MenuItem {
                label: "Ghostty",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Ghostty,
            },
            MenuItem {
                label: "Aerospace",
                description: "symlink config",
                kind: ItemKind::Sync,
                status: SyncStatus::Checking,
                id: ItemId::Aerospace,
            },
            MenuItem {
                label: "Corne Flash",
                description: "flash keyboard firmware",
                kind: ItemKind::Action,
                status: SyncStatus::Synced,
                id: ItemId::CorneFlash,
            },
            MenuItem {
                label: "Keyboard Layout",
                description: "view keymap layers",
                kind: ItemKind::Action,
                status: SyncStatus::Synced,
                id: ItemId::KeyboardLayout,
            },
            MenuItem {
                label: "Homebrew Sync",
                description: "sync installed packages",
                kind: ItemKind::Action,
                status: SyncStatus::Synced,
                id: ItemId::HomebrewSync,
            },
        ]
    }
}

fn check_symlink(link_path: &Path, expected_target: &Path) -> bool {
    std::fs::read_link(link_path)
        .map(|target| target == expected_target)
        .unwrap_or(false)
}

fn check_all_symlinks(pairs: &[(PathBuf, PathBuf)]) -> SyncStatus {
    let total = pairs.len();
    let synced = pairs
        .iter()
        .filter(|(link, target)| check_symlink(link, target))
        .count();

    if synced == total {
        SyncStatus::Synced
    } else if synced == 0 {
        SyncStatus::NotSynced
    } else {
        SyncStatus::Partial(synced, total)
    }
}

pub fn check_item_status(id: ItemId, repo_root: &Path) -> SyncStatus {
    match id {
        ItemId::Neovim => {
            let home = dirs_home();
            let nvim_dir = home.join(".config/nvim");
            let pairs = vec![
                (nvim_dir.join("init.lua"), repo_root.join("neovim/init.lua")),
                (nvim_dir.join("lua"), repo_root.join("neovim/lua")),
                (nvim_dir.join("snippets"), repo_root.join("neovim/snippets")),
            ];
            check_all_symlinks(&pairs)
        }
        ItemId::Tmux => {
            let home = dirs_home();
            let pairs = vec![(
                home.join(".tmux.conf"),
                repo_root.join("tmux/.tmux.conf"),
            )];
            check_all_symlinks(&pairs)
        }
        ItemId::Fish => {
            let home = dirs_home();
            let fish_dir = home.join(".config/fish");
            let mut pairs = vec![
                (
                    fish_dir.join("config.fish"),
                    repo_root.join("fish/config.fish"),
                ),
                (
                    fish_dir.join("fish_plugins"),
                    repo_root.join("fish/fish_plugins"),
                ),
                (
                    fish_dir.join("functions"),
                    repo_root.join("fish/functions"),
                ),
            ];
            if repo_root.join("fish/completions").exists() {
                pairs.push((
                    fish_dir.join("completions"),
                    repo_root.join("fish/completions"),
                ));
            }
            if repo_root.join("fish/conf.d").exists() {
                pairs.push((fish_dir.join("conf.d"), repo_root.join("fish/conf.d")));
            }
            if repo_root.join("fish/themes").exists() {
                pairs.push((fish_dir.join("themes"), repo_root.join("fish/themes")));
            }
            check_all_symlinks(&pairs)
        }
        ItemId::Claude => {
            let home = dirs_home();
            let claude_dir = home.join(".claude");
            let mut pairs = vec![(
                claude_dir.join("CLAUDE.md"),
                repo_root.join("claude/CLAUDE.md"),
            )];

            let skills_src = repo_root.join("claude/skills");
            if skills_src.exists() {
                if let Ok(entries) = std::fs::read_dir(&skills_src) {
                    for entry in entries.flatten() {
                        if entry.path().is_dir() {
                            let name = entry.file_name();
                            pairs.push((
                                claude_dir.join("skills").join(&name),
                                skills_src.join(&name),
                            ));
                        }
                    }
                }
            }

            let commands_src = repo_root.join("claude/commands");
            if commands_src.exists() {
                if let Ok(entries) = std::fs::read_dir(&commands_src) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().is_some_and(|e| e == "md") {
                            let name = entry.file_name();
                            pairs.push((
                                claude_dir.join("commands").join(&name),
                                commands_src.join(&name),
                            ));
                        }
                    }
                }
            }

            check_all_symlinks(&pairs)
        }
        ItemId::Ghostty => {
            let home = dirs_home();
            let pairs = vec![(
                home.join(".config/ghostty/config"),
                repo_root.join("ghostty/config"),
            )];
            check_all_symlinks(&pairs)
        }
        ItemId::Aerospace => {
            let home = dirs_home();
            let pairs = vec![(
                home.join(".aerospace.toml"),
                repo_root.join("aerospace/aerospace.toml"),
            )];
            check_all_symlinks(&pairs)
        }
        ItemId::Homebrew | ItemId::CorneFlash | ItemId::KeyboardLayout | ItemId::HomebrewSync => {
            SyncStatus::Synced
        }
    }
}

pub fn setup_command(id: ItemId, repo_root: &Path) -> Option<String> {
    let script = match id {
        ItemId::Homebrew => "homebrew/install.sh",
        ItemId::Neovim => "neovim/setup.sh",
        ItemId::Tmux => "tmux/setup.sh",
        ItemId::Fish => "fish/setup.sh",
        ItemId::Claude => "claude/setup.sh",
        ItemId::Ghostty => "ghostty/setup.sh",
        ItemId::Aerospace => return None,
        ItemId::CorneFlash | ItemId::KeyboardLayout | ItemId::HomebrewSync => return None,
    };
    Some(format!("bash {}", repo_root.join(script).display()))
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/Users/andy"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn all_items_has_correct_structure() {
        let items = MenuItem::all();
        assert_eq!(items.len(), 10);

        let sync_count = items.iter().filter(|i| i.kind == ItemKind::Sync).count();
        let action_count = items.iter().filter(|i| i.kind == ItemKind::Action).count();
        assert_eq!(sync_count, 7);
        assert_eq!(action_count, 3);
    }

    #[test]
    fn sync_items_start_as_checking() {
        let items = MenuItem::all();
        for item in &items {
            if item.kind == ItemKind::Sync {
                assert!(matches!(item.status, SyncStatus::Checking));
            }
        }
    }

    #[test]
    fn action_items_are_synced() {
        let items = MenuItem::all();
        for item in &items {
            if item.kind == ItemKind::Action {
                assert!(matches!(item.status, SyncStatus::Synced));
            }
        }
    }

    #[test]
    fn check_symlink_valid() {
        let dir = std::env::temp_dir().join("os_test_symlink");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let target = dir.join("target.txt");
        std::fs::write(&target, "hello").unwrap();

        let link = dir.join("link.txt");
        symlink(&target, &link).unwrap();

        assert!(check_symlink(&link, &target));
        assert!(!check_symlink(&link, &dir.join("wrong.txt")));
        assert!(!check_symlink(&dir.join("nonexistent"), &target));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn check_all_symlinks_full_match() {
        let dir = std::env::temp_dir().join("os_test_all_symlinks");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let t1 = dir.join("t1");
        let t2 = dir.join("t2");
        std::fs::write(&t1, "a").unwrap();
        std::fs::write(&t2, "b").unwrap();

        let l1 = dir.join("l1");
        let l2 = dir.join("l2");
        symlink(&t1, &l1).unwrap();
        symlink(&t2, &l2).unwrap();

        let pairs = vec![(l1.clone(), t1.clone()), (l2.clone(), t2.clone())];
        assert!(matches!(check_all_symlinks(&pairs), SyncStatus::Synced));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn check_all_symlinks_partial() {
        let dir = std::env::temp_dir().join("os_test_partial_symlinks");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let t1 = dir.join("t1");
        let t2 = dir.join("t2");
        std::fs::write(&t1, "a").unwrap();
        std::fs::write(&t2, "b").unwrap();

        let l1 = dir.join("l1");
        symlink(&t1, &l1).unwrap();

        let pairs = vec![
            (l1.clone(), t1.clone()),
            (dir.join("missing_link"), t2.clone()),
        ];
        assert!(matches!(
            check_all_symlinks(&pairs),
            SyncStatus::Partial(1, 2)
        ));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn check_all_symlinks_none() {
        let pairs = vec![
            (PathBuf::from("/nonexistent/a"), PathBuf::from("/nonexistent/b")),
            (PathBuf::from("/nonexistent/c"), PathBuf::from("/nonexistent/d")),
        ];
        assert!(matches!(check_all_symlinks(&pairs), SyncStatus::NotSynced));
    }

    #[test]
    fn setup_command_returns_correct_scripts() {
        let root = PathBuf::from("/repo");
        assert!(setup_command(ItemId::Neovim, &root)
            .unwrap()
            .contains("neovim/setup.sh"));
        assert!(setup_command(ItemId::Tmux, &root)
            .unwrap()
            .contains("tmux/setup.sh"));
        assert!(setup_command(ItemId::Fish, &root)
            .unwrap()
            .contains("fish/setup.sh"));
        assert!(setup_command(ItemId::Claude, &root)
            .unwrap()
            .contains("claude/setup.sh"));
        assert!(setup_command(ItemId::Ghostty, &root)
            .unwrap()
            .contains("ghostty/setup.sh"));
        assert!(setup_command(ItemId::Homebrew, &root)
            .unwrap()
            .contains("homebrew/install.sh"));
    }

    #[test]
    fn setup_command_returns_none_for_actions() {
        let root = PathBuf::from("/repo");
        assert!(setup_command(ItemId::CorneFlash, &root).is_none());
        assert!(setup_command(ItemId::KeyboardLayout, &root).is_none());
        assert!(setup_command(ItemId::HomebrewSync, &root).is_none());
        assert!(setup_command(ItemId::Aerospace, &root).is_none());
    }

    #[test]
    fn item_ids_are_unique() {
        let items = MenuItem::all();
        let ids: Vec<ItemId> = items.iter().map(|i| i.id).collect();
        for (i, id) in ids.iter().enumerate() {
            for (j, other) in ids.iter().enumerate() {
                if i != j {
                    assert_ne!(id, other);
                }
            }
        }
    }
}
