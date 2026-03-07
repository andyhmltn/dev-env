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

set -gx NVM_DIR $HOME/.nvm

function nvm
    set -l output (bass source ~/.nvm/nvm.sh --no-use ';' nvm $argv 2>&1)
    echo $output
    set -l node_ver (string match -r 'v[0-9]+\.[0-9]+\.[0-9]+' -- $output)
    if test -n "$node_ver"
        set -l node_dir "$NVM_DIR/versions/node/$node_ver/bin"
        if test -d "$node_dir"
            set -l cleaned_path
            for p in $PATH
                if not string match -q "$NVM_DIR/versions/node/*/bin" -- $p
                    set -a cleaned_path $p
                end
            end
            set -gx PATH $node_dir $cleaned_path
        end
    end
end

function nvm_find_nvmrc
    bass source ~/.nvm/nvm.sh --no-use ';' nvm_find_nvmrc
end

function load_nvm --on-variable="PWD"
    set -l default_node_version (nvm version default)
    set -l node_version (nvm version)
    set -l nvmrc_path (nvm_find_nvmrc)
    if test -n "$nvmrc_path"
        set -l nvmrc_node_version (nvm version (cat $nvmrc_path))
        if test "$nvmrc_node_version" = "N/A"
            nvm install (cat $nvmrc_path)
        else if test "$nvmrc_node_version" != "$node_version"
            nvm use $nvmrc_node_version
        end
    else if test "$node_version" != "$default_node_version"
        echo "Reverting to default Node version"
        nvm use default
    end
end

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

set --universal nvm_default_version 20

# pnpm
set -gx PNPM_HOME "/Users/andy/Library/pnpm"
if not string match -q -- $PNPM_HOME $PATH
  set -gx PATH "$PNPM_HOME" $PATH
end
# pnpm end

export PATH="$HOME/.local/bin:$PATH"
