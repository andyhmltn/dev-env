function review --description 'Review current branch vs default branch in tuicr via a squashed worktree'
    if not git rev-parse --is-inside-work-tree >/dev/null 2>&1
        echo "review: not in a git repository" >&2
        return 1
    end

    set -l branch (git rev-parse --abbrev-ref HEAD)
    if test "$branch" = HEAD
        echo "review: detached HEAD, no branch to review" >&2
        return 1
    end

    set -l default_branch
    set -l remote_head (git symbolic-ref --short refs/remotes/origin/HEAD 2>/dev/null)
    if test -n "$remote_head"
        set default_branch (string replace -r '^origin/' '' -- $remote_head)
    else if git show-ref --verify --quiet refs/heads/main
        set default_branch main
    else if git show-ref --verify --quiet refs/heads/master
        set default_branch master
    else
        echo "review: could not detect default branch" >&2
        return 1
    end

    if test "$branch" = "$default_branch"
        echo "review: already on $default_branch, nothing to review" >&2
        return 1
    end

    set -l repo_name (basename (git rev-parse --show-toplevel))
    set -l safe_branch (string replace -a / - -- $branch)
    set -l worktree_path (mktemp -d -t review-$repo_name-$safe_branch.XXXXXX)
    rm -rf $worktree_path

    echo "Setting up review worktree at $worktree_path ($default_branch <- $branch)..."
    if not git worktree add --quiet $worktree_path $default_branch
        echo "review: failed to create worktree" >&2
        return 1
    end

    pushd $worktree_path >/dev/null
    if not git merge --squash $branch
        echo "review: squash merge failed (conflicts). Worktree kept at $worktree_path for inspection." >&2
        popd >/dev/null
        return 1
    end

    tuicr -w
    set -l tuicr_status $status

    popd >/dev/null

    read --prompt-str "Remove review worktree at $worktree_path? [Y/n] " -l answer
    switch $answer
        case '' y Y yes
            git worktree remove --force $worktree_path
            echo "Removed."
        case '*'
            echo "Kept at $worktree_path"
    end

    return $tuicr_status
end
