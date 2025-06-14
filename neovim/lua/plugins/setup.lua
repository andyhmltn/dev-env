local actions = require("telescope.actions")

require("telescope").setup(
	{
		defaults = {
			mappings = { n = { ["q"] = actions.close } },
			layout_config = {
			},
		},
		pickers = {
			git_files = {
				theme = "ivy",
				path_display = { "truncate" }
			},
			find_files = { theme = "ivy", path_display = { "truncate" } },
			live_grep = { theme = "ivy", path_display = { "truncate" } }
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
			side = "right",
			width = 40
		}
	}
)

require("noice").setup({
	routes = {
		{
			view = 'notify',
			filter = { event = 'msg_showmode' }
		}

	},
	presets = {
		long_message_to_split = true,
		bottom_search = true,
		command_palette = true,
	},
})

require("notify").setup({
	background_colour = "#000000",
})

vim.defer_fn(function()
	vim.cmd('TSEnable highlight')
end, 100)

return {}
