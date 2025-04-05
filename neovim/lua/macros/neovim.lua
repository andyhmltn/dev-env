-- Open neovim config
vim.keymap.set('n', '<Leader>-cfg', function()
	vim.cmd('e ~/.config/nvim/init.lua')
end, { noremap = false })

-- Reload neovim config
vim.keymap.set('n', '<Leader>-ccfg', function()
	vim.cmd('luafile ~/.config/nvim/lua/macros/init.lua')
end, { noremap = false })

-- Run PlugInstall
vim.keymap.set('n', '<Leader>-pi', function()
	vim.cmd('PackerSync')
end, { noremap = false })

-- Spectre
vim.keymap.set('n', '<Leader>S', require("spectre").open, { noremap = false })

-- Lazygit
vim.keymap.set("n", "<leader>gg", ":!tmux new-window -c " .. vim.fn.getcwd() .. " -- lazygit <CR><CR>", { silent = true })

-- Open vertical split
vim.keymap.set("n", "<leader>W", "<C-w>v<C-W>w ", { silent = true })

-- Open horizontal split
vim.keymap.set("n", "<leader>V", function()
	vim.cmd('split')
	vim.cmd('wincmd w')
end, { silent = true })

-- Nvim tree shortcut
vim.keymap.set("n", "<leader>l", function()
	vim.cmd('NvimTreeFindFileToggle')
	vim.cmd('normal zz')
end, { silent = true })

-- Open todo in split
vim.keymap.set('n', '<Leader>-otd', function()
	vim.cmd('split')
	vim.cmd('wincmd w')
	vim.cmd('e ~/vimwiki/index.md')
	vim.cmd('resize 15')
end, { noremap = false })

-- Error diagnostics
vim.keymap.set("n", "<leader>e", vim.diagnostic.open_float, { desc = "Open floating diagnostic" })

-- Go to next
vim.keymap.set("n", "]g", function()
  vim.diagnostic.goto_next({ float = false })
end, { desc = "Go to next diagnostic" })

-- Go to previous
vim.keymap.set("n", "[g", function()
  vim.diagnostic.goto_prev({ float = false })
end, { desc = "Go to previous diagnostic" })
