
alias n="nvim $@"

eval "$(zoxide init zsh)"
source <(fzf --zsh)

alias zp="n ~/.zprofile && source ~/.zshrc"

neovimdev() {
	local dir
	dir=$(find ~/development -mindepth 1 -maxdepth 1 -type d | fzf)
	[ -n "$dir" ] && n "$dir"
}

# dev() {
# 	local dir
# 	dir=$(find ~/development -mindepth 1 -maxdepth 1 -type d | fzf)
# 	[ -n "$dir" ] && cd "$dir"
# }


alias nd=neovimdev

c() {
	local dir
	dir=$(find ~/development -mindepth 1 -maxdepth 1 -type d | fzf)
	[ -n "$dir" ] && cd "$dir"
}

alias go="/opt/homebrew/bin/go"
export PATH="$HOME/go/bin:$PATH"

export EDITOR=nvim
export VISUAL=nvim

