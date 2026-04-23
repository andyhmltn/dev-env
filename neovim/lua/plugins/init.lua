return {
	{ "nvim-lua/plenary.nvim", lazy = true },
	{ "nvim-tree/nvim-web-devicons", lazy = true },

	{
		"folke/tokyonight.nvim",
		lazy = false,
		priority = 1000,
		opts = { transparent = true },
		config = function(_, opts)
			require("tokyonight").setup(opts)
			vim.cmd.colorscheme("tokyonight-moon")
		end,
	},

	{
		"nvim-telescope/telescope.nvim",
		cmd = "Telescope",
		dependencies = {
			"nvim-lua/plenary.nvim",
			"nvim-telescope/telescope-frecency.nvim",
		},
		keys = {
			{ "<Leader><Esc>", ":Telescope projects<Cr>" },
			{ "<Leader>fb",    ":Telescope buffers<Cr>" },
			{ "<Leader>'",     ":Telescope live_grep<Cr>" },
			{ "<Leader>\"",    ":Telescope resume<Cr>" },
			{ "<Leader><Tab>", ":Telescope git_files<Cr>" },
		},
		config = function()
			local actions = require("telescope.actions")
			require("telescope").setup({
				defaults = {
					mappings = { n = { ["q"] = actions.close } },
					preview = { timeout = 100, debounce = 50 },
					file_ignore_patterns = { "node_modules", ".git/" },
					vimgrep_arguments = {
						"rg", "--color=never", "--no-heading", "--with-filename",
						"--line-number", "--column", "--smart-case", "--trim",
					},
				},
				pickers = {
					git_files  = { theme = "ivy", path_display = { "truncate" } },
					find_files = { theme = "ivy", path_display = { "truncate" } },
					live_grep  = { theme = "ivy", path_display = { "truncate" } },
					lsp_references = {
						layout_strategy = "vertical",
						layout_config = { width = 0.99, height = 0.99, preview_height = 0.6 },
						path_display = { "truncate" },
						show_line = true,
						initial_mode = "normal",
						cache_picker = { num_pickers = 10 },
						include_declaration = false,
						include_current_line = false,
						fname_width = 30,
						trim_text = true,
					},
				},
			})
		end,
	},
	{ "nvim-telescope/telescope-frecency.nvim", lazy = true },

	{
		"nvim-tree/nvim-tree.lua",
		cmd = { "NvimTreeToggle", "NvimTreeFindFile", "NvimTreeFindFileToggle", "NvimTreeOpen", "NvimTreeClose", "NvimTreeFocus" },
		dependencies = { "nvim-tree/nvim-web-devicons" },
		opts = {
			diagnostics = { enable = false },
			actions = { open_file = { quit_on_open = true } },
			respect_buf_cwd = true,
			view = { side = "right", width = 40 },
		},
	},

	{ "christoomey/vim-tmux-navigator", event = "VeryLazy" },

	{ "pasky/claude.vim", event = "VeryLazy" },

	{ "windwp/nvim-spectre", cmd = "Spectre", keys = { "<Leader>S" }, opts = {} },

	{ "kdheepak/lazygit.nvim", cmd = { "LazyGit", "LazyGitConfig", "LazyGitCurrentFile", "LazyGitFilter", "LazyGitFilterCurrentFile" } },

	{
		"pmizio/typescript-tools.nvim",
		name = "typescript-tools",
		ft = { "typescript", "typescriptreact", "javascript", "javascriptreact" },
		dependencies = { "nvim-lua/plenary.nvim", "neovim/nvim-lspconfig" },
		opts = {
			on_attach = function(client, bufnr)
				local bufopts = { noremap = true, silent = true, buffer = bufnr }
				vim.keymap.set("n", "gd", vim.lsp.buf.definition, bufopts)
				vim.keymap.set("n", "gr", function() require('telescope.builtin').lsp_references() end, bufopts)
				vim.keymap.set("n", "gi", vim.lsp.buf.implementation, bufopts)
				vim.keymap.set("n", "K", vim.lsp.buf.hover, bufopts)
				vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, bufopts)
				vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, bufopts)
				vim.keymap.set("n", "[g", function() vim.diagnostic.goto_prev({ float = false }) end, bufopts)
				vim.keymap.set("n", "]g", function() vim.diagnostic.goto_next({ float = false }) end, bufopts)
				client.server_capabilities.documentFormattingProvider = false
				client.server_capabilities.documentRangeFormattingProvider = false
			end,
			settings = {},
		},
	},

	{
		"echasnovski/mini.nvim",
		lazy = false,
		priority = 100,
		config = function()
			require("mini.starter").setup()
			require("mini.pairs").setup({})
		end,
	},
	{ "echasnovski/mini.pairs", lazy = true },

	{ "numToStr/Comment.nvim",           event = "VeryLazy", opts = {} },
	{ "gbprod/substitute.nvim",          event = "VeryLazy", opts = {} },
	{ "tpope/vim-surround",              event = "VeryLazy" },
	{ "tpope/vim-abolish",               event = "VeryLazy" },
	{ "michaeljsmith/vim-indent-object", event = "VeryLazy" },
	{ "ton/vim-bufsurf",                 cmd = { "BufSurfBack", "BufSurfForward" } },
	{ "vimwiki/vimwiki",                 ft = "markdown", cmd = { "VimwikiIndex", "VimwikiUISelect" } },
	{ "nicwest/vim-camelsnek",           cmd = { "Snek", "Camel", "CamelB", "Kebab", "Screm" } },

	{
		"folke/flash.nvim",
		event = "VeryLazy",
		opts = {
			modes = {
				char = {
					enabled = false,
					jump_labels = false,
					highlight = { backdrop = false, matches = false },
				},
			},
		},
	},

	{
		"neovim/nvim-lspconfig",
		event = { "BufReadPre", "BufNewFile" },
		config = function()
			vim.api.nvim_create_autocmd("LspAttach", {
				callback = function(args)
					local bufnr = args.buf
					local bufopts = { noremap = true, silent = true, buffer = bufnr }
					vim.keymap.set("n", "gd", vim.lsp.buf.definition, bufopts)
					vim.keymap.set("n", "gr", function() require('telescope.builtin').lsp_references() end, bufopts)
					vim.keymap.set("n", "gi", vim.lsp.buf.implementation, bufopts)
					vim.keymap.set("n", "K", vim.lsp.buf.hover, bufopts)
					vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, bufopts)
					vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, bufopts)
					vim.keymap.set("n", "[g", function() vim.diagnostic.goto_prev({ float = false }) end, bufopts)
					vim.keymap.set("n", "]g", function() vim.diagnostic.goto_next({ float = false }) end, bufopts)
					vim.api.nvim_create_user_command("Format", function()
						vim.lsp.buf.format()
					end, {})
				end,
			})

			vim.lsp.config.gopls = {
				cmd = { "gopls" },
				filetypes = { "go", "gomod", "gowork", "gotmpl" },
				root_markers = { "go.work", "go.mod", ".git" },
				settings = {},
			}

			vim.api.nvim_create_autocmd("BufWritePre", {
				pattern = { "*.go" },
				callback = function(args)
					vim.lsp.buf.format({ bufnr = args.buf })
				end,
			})

			vim.lsp.config.lua_ls = {
				cmd = { "lua-language-server" },
				filetypes = { "lua" },
				root_markers = { ".luarc.json", ".luarc.jsonc", ".luacheckrc", ".stylua.toml", "stylua.toml", "selene.toml", "selene.yml", ".git" },
				settings = {
					Lua = {
						diagnostics = { globals = { "vim" } },
						workspace = { checkThirdParty = false },
						telemetry = { enable = false },
					},
				},
			}

			vim.lsp.enable({ "gopls", "lua_ls" })
		end,
	},

	{
		"nvim-treesitter/nvim-treesitter",
		event = { "BufReadPost", "BufNewFile" },
		build = ":TSUpdate",
		config = function()
			require("nvim-treesitter.configs").setup({
				ensure_installed = {
					"lua", "go", "typescript", "tsx", "javascript", "json",
					"markdown", "markdown_inline", "html", "css", "yaml",
					"bash", "vim", "vimdoc",
				},
				auto_install = true,
				highlight = { enable = true },
				indent = { enable = true },
			})
		end,
	},

	{
		"folke/noice.nvim",
		event = "VeryLazy",
		dependencies = {
			"MunifTanjim/nui.nvim",
			{
				"rcarriga/nvim-notify",
				opts = {
					max_width = function()
						return math.floor(vim.o.columns * 0.40)
					end,
					top_down = false,
					stages   = "static",
					timeout  = 3000,
					background_colour = "#000000",
				},
				config = function(_, opts)
					local notify = require("notify")
					notify.setup(opts)
					vim.notify = notify
				end,
			},
		},
		opts = {
			routes = {
				{
					view = 'notify',
					filter = { event = 'msg_showmode' },
				},
			},
			presets = {
				long_message_to_split = true,
				bottom_search = true,
				command_palette = true,
			},
		},
	},

	{
		"luckasRanarison/tailwind-tools.nvim",
		ft = { "html", "css", "javascriptreact", "typescriptreact", "svelte", "vue" },
		opts = { server = { override = false } },
	},

	{ "bassamsdata/namu.nvim", cmd = "Namu", opts = {} },

	{
		"williamboman/mason.nvim",
		cmd = { "Mason", "MasonInstall", "MasonUpdate", "MasonUninstall", "MasonLog" },
		opts = {},
	},
	{
		"williamboman/mason-lspconfig.nvim",
		event = { "BufReadPre", "BufNewFile" },
		dependencies = { "williamboman/mason.nvim", "neovim/nvim-lspconfig" },
		opts = {
			ensure_installed = { "gopls" },
		},
	},

	{
		"stevearc/conform.nvim",
		event = { "BufWritePre" },
		cmd = { "ConformInfo" },
		opts = {
			format_on_save = function(bufnr)
				local bufname = vim.api.nvim_buf_get_name(bufnr)
				local dir = vim.fn.fnamemodify(bufname, ":h")
				local biome_config = vim.fs.find("biome.json", { path = dir, upward = true, type = "file" })
				if #biome_config > 0 then
					return {
						timeout_ms = 1000,
						lsp_fallback = false,
						formatters = { "biome" },
					}
				end
				return {
					timeout_ms = 1000,
					lsp_fallback = false,
				}
			end,
			formatters = {
				prettierd = { command = "prettierd", stdin = true },
			},
			formatters_by_ft = {
				javascript      = { "prettierd" },
				javascriptreact = { "prettierd" },
				typescript      = { "prettierd" },
				typescriptreact = { "prettierd" },
				json            = { "prettierd" },
				markdown        = { "prettierd" },
			},
		},
	},

	{ "stevearc/dressing.nvim", event = "VeryLazy", opts = {} },

	{
		"hrsh7th/nvim-cmp",
		event = "InsertEnter",
		dependencies = {
			"hrsh7th/cmp-nvim-lsp",
			"hrsh7th/cmp-buffer",
			"hrsh7th/cmp-path",
			"L3MON4D3/LuaSnip",
			"saadparwaiz1/cmp_luasnip",
			"rafamadriz/friendly-snippets",
		},
		config = function()
			local cmp = require("cmp")
			local luasnip = require("luasnip")

			luasnip.config.set_config({
				history = true,
				updateevents = "TextChanged,TextChangedI",
				enable_autosnippets = true,
			})

			require("luasnip-config")

			require("luasnip.loaders.from_vscode").lazy_load({
				paths = { vim.fn.stdpath("config") .. "/snippets" },
			})

			vim.keymap.set({ "i", "s" }, "<C-n>", function()
				if luasnip.expand_or_jumpable() then
					luasnip.expand_or_jump()
				end
			end, { silent = true })

			vim.keymap.set({ "i", "s" }, "<C-p>", function()
				if luasnip.jumpable(-1) then
					luasnip.jump(-1)
				end
			end, { silent = true })

			cmp.setup({
				snippet = {
					expand = function(args) luasnip.lsp_expand(args.body) end,
				},
				confirmation = { completeopt = 'menu,menuone,noinsert' },
				formatting = {
					format = function(entry, vim_item)
						vim_item.menu = ({
							nvim_lsp = "[LSP]",
							luasnip  = "[Snippet]",
							buffer   = "[Buffer]",
							path     = "[Path]",
						})[entry.source.name]
						return vim_item
					end,
				},
				mapping = {
					["<Up>"] = cmp.mapping(function(fallback)
						if cmp.visible() then cmp.select_prev_item() else fallback() end
					end, { "i", "s" }),
					["<Down>"] = cmp.mapping(function(fallback)
						if cmp.visible() then cmp.select_next_item() else fallback() end
					end, { "i", "s" }),
					["<Tab>"] = cmp.mapping(function(fallback)
						if cmp.visible() then
							cmp.select_next_item()
						elseif luasnip.expand_or_jumpable() then
							luasnip.expand_or_jump()
						else
							fallback()
						end
					end, { "i", "s" }),
					["<S-Tab>"] = cmp.mapping(function(fallback)
						if cmp.visible() then
							cmp.select_prev_item()
						elseif luasnip.jumpable(-1) then
							luasnip.jump(-1)
						else
							fallback()
						end
					end, { "i", "s" }),
					["<CR>"] = cmp.mapping.confirm({ select = true }),
				},
				sources = cmp.config.sources({
					{ name = "nvim_lsp" },
					{ name = "luasnip" },
					{ name = "buffer" },
					{ name = "path" },
				}),
			})
		end,
	},

	{
		"afewyards/codereview.nvim",
		dependencies = { "nvim-lua/plenary.nvim" },
		cmd = {
			"CodeReview", "CodeReviewAI", "CodeReviewAIFile", "CodeReviewStart",
			"CodeReviewSubmit", "CodeReviewApprove", "CodeReviewOpen",
			"CodeReviewPipeline", "CodeReviewComments", "CodeReviewFiles",
			"CodeReviewToggleScroll", "CodeReviewCommits",
		},
		opts = {},
	},
}
