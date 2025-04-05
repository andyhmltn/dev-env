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

-- General options
vim.opt.relativenumber = true
vim.opt.number = true
vim.opt.cmdheight = 2
vim.opt.swapfile = false
vim.opt.hlsearch = false
vim.opt.timeoutlen = 5000

-- Shell options (prefer last set shell)
vim.opt.shell = "/bin/bash -i"

-- Set leader key
vim.g.mapleader = " "

-- Environment variables or plugin-specific globals
vim.g.claude_map_send_chat_message = "<leader>cs"


vim.g.NERDTreeMinimalMenu = 1

-- Registers (setting contents of named registers)
vim.fn.setreg("t", "0witype ")
vim.fn.setreg("p", "elc$, ")

-- Key mappings
local keymap = vim.keymap.set
local opts = { noremap = true, silent = true }

-- Prevent unnamed register overwrite on paste in visual mode
keymap("x", "p", [["pgv\"" .. vim.v.register .. "y"]], { expr = true, silent = true })

-- Black hole register for 'd' and 'c' (no yank)
keymap("n", "d", '"_d', opts)
keymap("n", "c", '"_c', opts)
keymap("v", "c", '"_c', opts)

-- Recenter on search/navigation
keymap("n", "n", "nzz", opts)
keymap("n", "N", "Nzz", opts)
keymap("n", "*", "*zz", opts)
keymap("n", "}", "}zz", opts)
keymap("n", "{", "{zz", opts)

-- Set clipboard
vim.opt.clipboard = "unnamedplus"

-- UltiSnips
vim.g.UltiSnipsExpandTrigger = "<tab>"
vim.g.UltiSnipsJumpForwardTrigger = "<c-b>"
vim.g.UltiSnipsJumpBackwardTrigger = "<c-z>"

vim.diagnostic.config({
  virtual_text = true,
  float = {
    source = "always",
  },
})

