vim.keymap.set("n", "<c-s>", require('substitute').operator, { noremap = true })
vim.keymap.set("n", "<leader>-ss", require('substitute').line, { noremap = true })
vim.keymap.set("n", "<leader>-S", require('substitute').eol, { noremap = true })
vim.keymap.set("x", "s", require('substitute').visual, { noremap = true })

