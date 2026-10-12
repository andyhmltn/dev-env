function fish_prompt --description 'Write a compact shell prompt'
    set -l last_status $status
    set -l location (basename "$PWD")

    if string match -rq '.*/([^/]+)/worktrees/.*' -- "$PWD"
        set location (string replace -r '.*/([^/]+)/worktrees/.*' '$1' -- "$PWD")
    else if test "$PWD" = "$HOME"
        set location '~'
    end

    set -l branch (command git branch --show-current 2>/dev/null)
    if test -n "$branch"
        set -l branch_length (string length -- "$branch")
        if test "$branch_length" -gt 28
            set branch (string sub --length 28 -- "$branch")…
        end
    end

    set_color brcyan
    echo -n "$location"

    if test -n "$branch"
        set_color brblack
        echo -n ' '
        set_color brmagenta
        echo -n "$branch"
    end

    echo -n ' '
    if test "$last_status" -eq 0
        set_color brgreen
    else
        set_color brred
    end
    echo -n '❯ '
    set_color normal
end
