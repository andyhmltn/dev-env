--
-- Open neovim config
vim.keymap.set('n', '<Leader>-cfg', function()
	vim.cmd('e ~/.config/nvim/init.vim')
end, { noremap = false })

-- Reload neovim config
vim.keymap.set('n', '<Leader>-ccfg', function()
	vim.cmd('source ~/.config/nvim/init.vim')
end, { noremap = false })

-- Run PlugInstall
vim.keymap.set('n', '<Leader>-pi', function()
	vim.cmd('PlugInstall')
end, { noremap = false })

-- Spectre
vim.keymap.set('n', '<Leader>S', require("spectre").open, { noremap = false })

-- Lazygit
vim.keymap.set("n", "<leader>gg", ":!tmux new-window -c " .. vim.fn.getcwd() .. " -- lazygit <CR><CR>", { silent = true })

