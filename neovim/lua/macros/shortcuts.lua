-- Bracket shortcuts
vim.keymap.set('n', '<Leader>-gc', '/{}<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>-gb', '/()<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>-gs', '/[]<Cr>', { noremap = false })
 
--  Buffer surfing
vim.keymap.set('n', '<Leader>-gtl', ':BufSurfBack<CR>', { noremap = false })
vim.keymap.set('n', '<Leader>-btl', ':BufSurfForward<CR>', { noremap = false })

-- Symbols
vim.keymap.set("n", "<leader>t",":Namu symbols<cr>" , {
	desc = "Jump to LSP symbol",
	silent = true,
})
 
-- Add missing imports
vim.keymap.set("n", "<leader>i",":TSToolsAddMissingImports<cr>" , {
	desc = "Auto import missing imports",
	silent = true,
})
 
-- Space around current cursor
vim.keymap.set('n', '<Leader>-k', 'o<Esc>kO<Esc>j', { noremap = false })

