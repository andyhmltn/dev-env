eval (/opt/homebrew/bin/brew shellenv)

# Auto-start tmux if not already inside tmux
if status is-interactive
    and not set -q TMUX
    and command -q tmux
    exec tmux new-session -A -s main
end

alias zp 'nvim ~/.config/fish/config.fish; source ~/.config/fish/config.fish'
alias n 'nvim $argv'

function c
    set -l dir (find ~/dev -mindepth 1 -maxdepth 1 -type d | fzf)
    if test -n "$dir"
        cd "$dir"
    end
end

function nd
    set -l dir (find ~/dev -mindepth 1 -maxdepth 1 -type d | fzf)
    if test -n "$dir"
        nvim "$dir"
    end
end

fnm env --use-on-cd --shell fish | source

alias aws-login 'aws sso login --sso-session my-sso'
alias gg 'lazygit'
alias q 'tmux kill-pane'
alias t 'tmux'
alias p 'pnpm $argv'
alias cf 'npx claude-flow@alpha $argv'
abbr :q 'tmux kill-pane'
abbr ':q!' 'tmux kill-pane'
alias scripts 'cat package.json | jq .scripts'
alias build-raw 'xcodebuild -scheme "CoachFit-RawCoaching" -configuration Release -destination "generic/platform=iOS"'


eval (zoxide init fish | source)
alias cd z
alias gg lazygit

# pnpm
set -gx PNPM_HOME "/Users/andy/Library/pnpm"
if not string match -q -- $PNPM_HOME $PATH
  set -gx PATH "$PNPM_HOME" $PATH
end
# pnpm end

export PATH="$HOME/.local/bin:$PATH"
