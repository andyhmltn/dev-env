alias n="nvim $@"

eval "$(zoxide init zsh)"
source <(fzf --zsh)

alias zp="n ~/.zprofile && source ~/.zshrc"

neovimdev() {
	local dir
	dir=$(find ~/development -mindepth 1 -maxdepth 1 -type d | fzf)
	[ -n "$dir" ] && n "$dir"
}

alias dev=neovimdev
