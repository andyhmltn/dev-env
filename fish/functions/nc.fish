function nc --description 'nflow config'
    if set -q TMUX
        tmux display-popup -E -w 90% -h 85% 'tmux new-session -A -s config "tmux set-option status off; exec nvim ~/.config/nflow/config.toml"'
    else
        nvim ~/.config/nflow/config.toml
    end
end

