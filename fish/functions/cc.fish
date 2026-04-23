function cc --description 'Pick a ~/dev directory with fzf and launch claude there'
    set -l dir (fd --type d --max-depth 1 . ~/dev | fzf --prompt='~/dev > ')
    or return

    cd $dir
    or return

    claude --dangerously-skip-permissions $argv
end
