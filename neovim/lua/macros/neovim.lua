-- Spectre
vim.keymap.set('n', '<Leader>S', require("spectre").open, { noremap = false })

-- Lazygit
-- vim.keymap.set("n", "<leader>gg", ":!tmux new-window -c " .. vim.fn.getcwd() .. " -- lazygit <CR><CR>", { silent = true })
vim.keymap.set("n", "<leader>gg", ":!tmux new-window lazygit <CR><CR>", { silent = true })

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
	vim.cmd('resize 40')
end, { noremap = false })

vim.keymap.set('n', '<c-t>', function()
	vim.cmd('split')
	vim.cmd('wincmd w')
	vim.cmd('e ~/vimwiki/index.md')
	vim.cmd('resize 40')
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
