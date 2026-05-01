require('settings')

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

require("lazy").setup("plugins", {
  performance = {
    rtp = {
      disabled_plugins = {
        "gzip", "tarPlugin", "zipPlugin", "tohtml", "tutor", "netrwPlugin",
      },
    },
  },
})

vim.api.nvim_create_autocmd("User", {
  pattern = "VeryLazy",
  callback = function()
    require('plugins.custom.grep-in-folder')
    require('plugins.custom.smart-paste-arg')
    require('plugins.custom.wildcard-search')
    require('plugins.custom.eslint').setup()
    require('macros')
  end,
})

vim.api.nvim_create_autocmd("SwapExists", {
  callback = function()
    vim.v.swapchoice = 'e'
  end,
})

vim.g.vimwiki_list = { {
  path = '~/vimwiki/',
  syntax = 'markdown',
  ext = 'md',
} }

vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    vim.cmd("%argdelete")
  end,
})

vim.o.guifont = "Iosevka Nerd Font:h12"

vim.opt.shortmess = "TAF"
