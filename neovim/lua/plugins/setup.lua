local actions = require("telescope.actions")

require("telescope").setup(
	{
		defaults = {
			mappings = { n = { ["q"] = actions.close } }
		},
		pickers = {
			git_files = { theme = "ivy" },
			find_files = { theme = "ivy" },
			live_grep = { theme = "ivy" }
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

require("noice").setup({
  presets = {
    long_message_to_split = true, -- Redirect long messages to a split window
    bottom_search = true,         -- Optional: Classic bottom cmdline for search
    command_palette = true,       -- Optional: Position cmdline and popupmenu together
  },
})

require("notify").setup({
	background_colour = "#000000",
})

vim.defer_fn(function()
	vim.cmd('TSEnable highlight')
end, 100)

return {}
