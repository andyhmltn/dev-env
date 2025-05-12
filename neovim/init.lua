require('settings')

-- Lazy.nvim setup
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git", "clone", "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    lazypath
  })
end
vim.opt.rtp:prepend(lazypath)

vim.opt.cmdheight = 0
vim.opt.shortmess:append("TF")

require("lazy").setup("plugins")


-- Custom settings
require('plugins.setup')

require('plugins.custom.grep-in-folder')
require('plugins.custom.smart-paste-arg')
require('plugins.custom.wildcard-search')

require('macros')

-- Disable annoying popup about swap files
vim.api.nvim_create_autocmd("SwapExists", {
  callback = function()
    vim.v.swapchoice = 'e'
  end,
})

-- Vimwiki config
vim.g.vimwiki_list = {{
  path = '~/vimwiki/',
  syntax = 'markdown',
  ext = 'md',
}}

-- Colorscheme
-- vim.cmd [[
--   colorscheme catppuccin-macchiato
--   set background=dark
-- ]]

require("catppuccin").setup({
  transparent_background = true,
})
vim.cmd.colorscheme "catppuccin-macchiato"

-- Font (GUI only)
vim.o.guifont = "Iosevka Nerd Font:h12"

-- Suppress swap file messages
vim.o.shortmess = "A"

-- Prettier command with Coc
vim.api.nvim_create_user_command("Prettier", function()
  vim.fn["CocAction"]("runCommand", "prettier.formatFile")
end, {})
