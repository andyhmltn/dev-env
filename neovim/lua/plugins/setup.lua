local builtin = require("telescope.builtin")
local utils = require("telescope.utils")
local actions = require("telescope.actions")

require("telescope").setup(
	{
		defaults = {
			mappings = { n = { ["q"] = actions.close } }
		},
		pickers = {
			find_files = { theme = "dropdown" },
			live_grep = { theme = "dropdown" }
		}
	}
)



require("Comment").setup()
require("substitute").setup()
require("tailwind-tools").setup()
require("namu").setup()
require("mini.starter").setup()
require("spectre").setup()

-- Set up mini.pairs with custom mappings
require("mini.pairs").setup({})

require("flash").setup(
	{
		modes = {
			char = {
				enabled = false,
				jump_labels = false,
				highlight = {
					backdrop = false,
					matches = false
				}
			}
		}
	}
)

require("nvim-tree").setup(
	{
		diagnostics = {
			enable = false
		},
		actions = {
			open_file = {
				quit_on_open = true
			}
		},
		respect_buf_cwd = true,
		view = {
			side = "left",
			width = 40
		}
	}
)

vim.defer_fn(function()
	vim.cmd('TSEnable highlight')
end, 100)

return {}
