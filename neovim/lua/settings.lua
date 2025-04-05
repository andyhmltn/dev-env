-- disable netrw at the very start of your init.lua (strongly advised)
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1
 
vim.opt.showtabline = 2
vim.o.tabline = "./%t"
 
vim.opt.termguicolors = true
 
-- formatting settings
vim.opt.smartindent = true
 
-- Searching options
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.inccommand = "split"
 
 
-- Enable enter as next target for vim Leap
vim.g.leap_target_next_key = "<CR>"

