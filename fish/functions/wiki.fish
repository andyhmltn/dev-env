function wiki --description 'Open vimwiki in a persistent tmux popup'
    if set -q TMUX
        tmux display-popup -E -w 90% -h 85% 'tmux new-session -A -s wiki "tmux set-option status off; exec nvim ~/vimwiki/index.wiki"'
    else
        nvim ~/vimwiki/index.wiki
    end
end
