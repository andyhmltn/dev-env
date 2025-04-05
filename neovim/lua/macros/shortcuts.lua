-- Bracket shortcuts
vim.keymap.set('n', '<Leader>-gc', '/{}<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>-gb', '/()<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>-gs', '/[]<Cr>', { noremap = false })
 
--  Buffer surfing
vim.keymap.set('n', '<Leader>-gtl', ':BufSurfBack<CR>', { noremap = false })
vim.keymap.set('n', '<Leader>-btl', ':BufSurfForward<CR>', { noremap = false })
