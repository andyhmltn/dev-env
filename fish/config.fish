eval (/opt/homebrew/bin/brew shellenv)

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

set -gx NVM_DIR $HOME/.nvm

alias aws-login 'aws sso login --sso-session my-sso'
alias gg 'lazygit'
alias q 'tmux kill-pane'
alias t 'tmux'
alias p 'pnpm $argv'
abbr :q 'tmux kill-pane'
abbr ':q!' 'tmux kill-pane'
alias scripts 'cat package.json | jq .scripts'


# eval (zoxide init fish | source)
# alias cd z
alias gg lazygit

set --universal nvm_default_version 20
