---
name: merge-worktree
description: Merge the currently active git worktree branch into main and clean up the worktree.
---

# Merge Worktree to Main

## Overview

This skill merges the current worktree's branch into main and optionally cleans up the worktree.

## Prerequisites

- Must be in a git worktree (not the main working directory)
- All changes must be committed
- The branch must be ready to merge (tests passing, etc.)

## Workflow

### Step 1: Validate Worktree State

```bash
git worktree list
git status
git log --oneline -5
```

**Check:**
- Confirm you're in a worktree (not main working directory)
- Ensure working directory is clean (no uncommitted changes)
- Note the current branch name

### Step 2: Get Main Working Directory

```bash
git worktree list
```

Find the main worktree path (typically without a branch suffix in the path).

### Step 3: Fetch and Update Main

```bash
git fetch origin main
```

### Step 4: Merge Branch into Main

From the main worktree directory:

```bash
cd <main-worktree-path>
git checkout main
git pull origin main
git merge --no-ff <branch-name> -m "Merge <branch-name> into main"
```

**Conflict handling:**
If conflicts occur, stop and ask the user how to proceed.

### Step 5: Push to Remote

```bash
git push origin main
```

### Step 6: Clean Up (Optional)

Ask the user if they want to remove the worktree:

```bash
git worktree remove <worktree-path>
git branch -d <branch-name>
```

## Safety Checks

Before merging, verify:
1. Working directory is clean
2. Branch has commits ahead of main
3. No uncommitted changes exist
4. User confirms the merge

## Error Handling

- If not in a worktree: Inform user and exit
- If uncommitted changes: Ask user to commit or stash
- If merge conflicts: Stop and let user resolve manually
- If push fails: Check remote access and branch protection
