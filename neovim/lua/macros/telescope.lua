vim.keymap.set('n', '<Leader><Esc>', ':Telescope projects<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>fb', ':Telescope buffers<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>"', ':Telescope live_grep<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader>\'', ':Telescope resume<Cr>', { noremap = false })
vim.keymap.set('n', '<Leader><Tab>', ':Telescope git_files<Cr>', { noremap = false })

function search_files_in_current_buffer_dir()
	require('telescope.utils')
	require('telescope.builtin').find_files({ cwd = vim.fn.expand('%:p:h') })
end

vim.keymap.set('n', '<Leader>ff', search_files_in_current_buffer_dir, { noremap = false })

