function wiki --description 'Open vimwiki index in a new tmux window'
    if set -q TMUX
        tmux new-window -n wiki "nvim ~/vimwiki/index.md"
    else
        nvim ~/vimwiki/index.md
    end
end
