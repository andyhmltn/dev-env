vim.keymap.set("n", "<c-s>", function() require('substitute').operator() end, { noremap = true })
vim.keymap.set("n", "<leader>-ss", function() require('substitute').line() end, { noremap = true })
vim.keymap.set("n", "<leader>-S", function() require('substitute').eol() end, { noremap = true })
vim.keymap.set("x", "s", function() require('substitute').visual() end, { noremap = true })
