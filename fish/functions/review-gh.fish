function review-gh --description 'Review current branch with tuicr, post as inline GitHub PR review'
    if not git rev-parse --is-inside-work-tree >/dev/null 2>&1
        echo "review-gh: not in a git repository" >&2
        return 1
    end

    set -l branch (git rev-parse --abbrev-ref HEAD)
    if test "$branch" = HEAD
        echo "review-gh: detached HEAD, no branch to review" >&2
        return 1
    end

    set -l event COMMENT
    set -l extra_body ""
    set -l dry_run 0
    set -l argv_remaining

    while test (count $argv) -gt 0
        switch $argv[1]
            case --approve -a
                set event APPROVE
            case --request-changes -r
                set event REQUEST_CHANGES
            case --comment -c
                set event COMMENT
            case --body -b
                set extra_body $argv[2]
                set argv $argv[2..-1]
            case --dry-run -n
                set dry_run 1
            case '*'
                set -a argv_remaining $argv[1]
        end
        set argv $argv[2..-1]
    end

    if not command -q gh
        echo "review-gh: gh CLI not installed" >&2
        return 1
    end
    if not gh auth status >/dev/null 2>&1
        echo "review-gh: gh not authenticated (run: gh auth login)" >&2
        return 1
    end

    set -l pr_json (gh pr view --json number,headRefOid,baseRefName,url 2>/dev/null)
    if test -z "$pr_json"
        echo "review-gh: no PR found for branch $branch (push and open one first)" >&2
        return 1
    end

    set -l pr_number (echo $pr_json | jq -r .number)
    set -l pr_head_sha (echo $pr_json | jq -r .headRefOid)
    set -l pr_base (echo $pr_json | jq -r .baseRefName)
    set -l pr_url (echo $pr_json | jq -r .url)

    set -l owner_repo (string match -r 'github\.com/([^/]+)/([^/]+)/pull/' $pr_url)
    if test (count $owner_repo) -lt 3
        echo "review-gh: could not parse owner/repo from $pr_url" >&2
        return 1
    end
    set -l owner $owner_repo[2]
    set -l repo $owner_repo[3]

    set -l local_sha (git rev-parse HEAD)
    if test "$local_sha" != "$pr_head_sha"
        echo "review-gh: local HEAD ($local_sha) does not match PR head ($pr_head_sha)" >&2
        echo "  Push your branch first (git push), then retry." >&2
        return 1
    end

    set -l repo_name (basename (git rev-parse --show-toplevel))
    set -l safe_branch (string replace -a / - -- $branch)
    set -l worktree_path (mktemp -d -t review-gh-$repo_name-$safe_branch.XXXXXX)
    rm -rf $worktree_path

    echo "Setting up review worktree at $worktree_path ($pr_base <- $branch)..."
    if not git worktree add --quiet $worktree_path $pr_base
        echo "review-gh: failed to create worktree" >&2
        return 1
    end

    set -l output_file (mktemp -t tuicr-review-XXXXXX.md)

    pushd $worktree_path >/dev/null
    if not git merge --squash $branch
        echo "review-gh: squash merge failed (conflicts). Worktree kept at $worktree_path." >&2
        popd >/dev/null
        rm -f $output_file
        return 1
    end

    echo "Launching tuicr. Press 'e' to export your review when done."
    tuicr -w --stdout >$output_file
    set -l tuicr_status $status
    popd >/dev/null

    if test $tuicr_status -ne 0
        echo "review-gh: tuicr exited with status $tuicr_status" >&2
        rm -f $output_file
        git worktree remove --force $worktree_path
        return $tuicr_status
    end

    if not test -s $output_file
        echo "review-gh: no review exported (empty output). Nothing posted." >&2
        rm -f $output_file
        git worktree remove --force $worktree_path
        return 0
    end

    echo
    echo "--- Captured review ($output_file) ---"
    cat $output_file
    echo "--- End ---"
    echo

    if test $dry_run -eq 1
        echo "Dry run: would post as $event to $pr_url"
    else
        read --prompt-str "Post as $event to $pr_url? [Y/n/edit] " -l answer
        switch $answer
            case '' y Y yes
            case e edit
                set -l editor (set -q EDITOR; and echo $EDITOR; or echo nvim)
                $editor $output_file
            case '*'
                echo "Cancelled. Output kept at $output_file"
                git worktree remove --force $worktree_path
                return 0
        end
    end

    set -l script_dir (dirname (status filename))
    set -l py_script $script_dir/_review_gh_post.py

    set -l py_args --input $output_file \
                   --owner $owner --repo $repo \
                   --pr $pr_number --head-sha $pr_head_sha \
                   --event $event
    if test -n "$extra_body"
        set -a py_args --body $extra_body
    end
    if test $dry_run -eq 1
        set -a py_args --dry-run
    end

    python3 $py_script $py_args
    set -l post_status $status

    if test $post_status -eq 0; and test $dry_run -eq 0
        rm -f $output_file
    else
        echo "Output preserved at $output_file"
    end

    git worktree remove --force $worktree_path
    return $post_status
end
