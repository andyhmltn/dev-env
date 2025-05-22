return {
	-- Telescope and deps
	{ "nvim-lua/plenary.nvim" },
	{ "nvim-telescope/telescope.nvim" },
	{ "nvim-telescope/telescope-frecency.nvim" },

	-- File explorer
	{ "nvim-tree/nvim-tree.lua" },
	{ "nvim-tree/nvim-web-devicons" },

	-- Tmux navigation
	{ "christoomey/vim-tmux-navigator" },

	-- AI
	{ "pasky/claude.vim" },

	-- Dev tools
	{ "windwp/nvim-spectre" },
	{ "kdheepak/lazygit.nvim" },
	{ "pmizio/typescript-tools.nvim",          name = "typescript-tools" },

	-- Mini plugins
	{ "echasnovski/mini.nvim" },
	{ "echasnovski/mini.pairs" },

	-- Editor enhancements
	{ "numToStr/Comment.nvim" },
	{ "gbprod/substitute.nvim" },
	{ "SirVer/ultisnips" },
	{ "tpope/vim-surround" },
	{ "tpope/vim-abolish" },
	{ "michaeljsmith/vim-indent-object" },
	{ "ton/vim-bufsurf" },
	{ "vimwiki/vimwiki" },
	{ "nicwest/vim-camelsnek" },
	{ "folke/flash.nvim" },

	-- LSP
	{ "neovim/nvim-lspconfig" },

	-- Treesitter
	{
		"nvim-treesitter/nvim-treesitter",
		-- build = ":TSUpdate"
	},

	-- Themes
	{ "catppuccin/nvim",                    name = "catppuccin" },

	{
		"folke/noice.nvim",
		event = "VeryLazy",
		opts = {},
		dependencies = {
			"MunifTanjim/nui.nvim",
			"rcarriga/nvim-notify",
		}
	},


	-- Tailwind
	{ "luckasRanarison/tailwind-tools.nvim" },

	-- Custom plugins
	{ "bassamsdata/namu.nvim" },

	-- Mason: external tool manager for LSP servers, formatters, linters
	{
		"williamboman/mason.nvim",
		config = function()
			require("mason").setup()
		end
	},
	-- Mason-lspconfig: bridge between mason.nvim and lspconfig
	{
		"williamboman/mason-lspconfig.nvim",
		dependencies = { "neovim/nvim-lspconfig" },
		config = function()
			require("mason-lspconfig").setup({
				ensure_installed = { "gopls" },
			})
		end
	},
	-- Nvim-lspconfig: official LSP client config
	{
		"neovim/nvim-lspconfig",
		config = function()
			local lspconfig = require("lspconfig")
			local on_attach = function(client, bufnr)
				-- Keymaps for LSP
				local bufopts = { noremap = true, silent = true, buffer = bufnr }
				vim.keymap.set("n", "gd", vim.lsp.buf.definition, bufopts)
				vim.keymap.set("n", "gr", vim.lsp.buf.references, bufopts)
				vim.keymap.set("n", "gi", vim.lsp.buf.implementation, bufopts)
				vim.keymap.set("n", "K", vim.lsp.buf.hover, bufopts)
				vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, bufopts)
				vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, bufopts)
				vim.keymap.set("n", "[g", function() vim.diagnostic.goto_prev({ float = false }) end,
					bufopts)
				vim.keymap.set("n", "]g", function() vim.diagnostic.goto_next({ float = false }) end,
					bufopts)
				-- Optional: Format on save, or define a command
				vim.api.nvim_create_user_command("Format", function()
					vim.lsp.buf.format()
				end, {})
			end

			lspconfig.gopls.setup({
				on_attach = function(client, bufnr)
					-- Create a command or auto-cmd to format on save
					vim.api.nvim_create_autocmd("BufWritePre", {
						buffer = bufnr, -- only for current buffer
						callback = function()
							vim.lsp.buf.format({ bufnr = bufnr })
						end,
					})

					on_attach(client, bufnr)
				end,
				settings = {
				}
			})

			lspconfig.lua_ls.setup({
				on_attach = on_attach,
				settings = {
					Lua = {
						diagnostics = { globals = { "vim" } },
						workspace = { checkThirdParty = false },
						telemetry = { enable = false },
					}
				}
			})
		end
	},
	{
		"pmizio/typescript-tools.nvim",
		name = "typescript-tools",
		dependencies = { "nvim-lua/plenary.nvim", "neovim/nvim-lspconfig" },
		opts = {
			-- See the plugin's README for other available settings
			on_attach = function(client, bufnr)
				-- Keymaps for LSP
				local bufopts = { noremap = true, silent = true, buffer = bufnr }
				vim.keymap.set("n", "gd", vim.lsp.buf.definition, bufopts)
				vim.keymap.set("n", "gr", vim.lsp.buf.references, bufopts)
				vim.keymap.set("n", "gi", vim.lsp.buf.implementation, bufopts)
				vim.keymap.set("n", "K", vim.lsp.buf.hover, bufopts)
				vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, bufopts)
				vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, bufopts)
				vim.keymap.set("n", "[g", function() vim.diagnostic.goto_prev({ float = false }) end,
					bufopts)
				vim.keymap.set("n", "]g", function() vim.diagnostic.goto_next({ float = false }) end,
					bufopts)

				client.server_capabilities.documentFormattingProvider = false
				client.server_capabilities.documentRangeFormattingProvider = false
			end,

			settings = {
			},
		}
	},
	{
		"stevearc/conform.nvim",
		opts = {
			  format_on_save = {
			    lsp_fallback = false,
			    timeout_ms   = 1000,
			    format_on_save       = true,
			  },
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
	{
		'stevearc/dressing.nvim',
		opts = {},
	},
	{
		"hrsh7th/nvim-cmp",
		dependencies = {
			"hrsh7th/cmp-nvim-lsp", -- LSP source for nvim-cmp
			"hrsh7th/cmp-buffer", -- Buffer completions
			"hrsh7th/cmp-path", -- Path completions
			"L3MON4D3/LuaSnip", -- Snippet engine
			"saadparwaiz1/cmp_luasnip",
		},
		config = function()
			local cmp = require("cmp")

			cmp.setup({
				confirmation = { completeopt = 'menu,menuone,noinsert' },
				mapping = {
					["<Up>"] = cmp.mapping(function(fallback)
						if cmp.visible() then
							cmp.select_prev_item()
						else
							fallback()
						end
					end, { "i", "s" }),
					["<Down>"] = cmp.mapping(function(fallback)
						if cmp.visible() then
							cmp.select_next_item()
						else
							fallback()
						end
					end, { "i", "s" }),
					["<Tab>"] = cmp.mapping(function(fallback)
						if cmp.visible() then
							cmp.select_next_item()
						else
							fallback()
						end
					end, { "i", "s" }),
					["<CR>"] = cmp.mapping.confirm({ select = true }),
				},
				sources = cmp.config.sources({
					{ name = "nvim_lsp" },
					-- { name = "luasnip" },
					{ name = "buffer" },
					{ name = "path" },
				}),
			})
		end
	},
	{
		"davidmh/mdx.nvim",
		config = true,
		dependencies = { "nvim-treesitter/nvim-treesitter" }
	},
	{
	  "yetone/avante.nvim",
	  event = "VeryLazy",
	  version = false, -- Never set this value to "*"! Never!
	  opts = {
	    provider = "claude",
	  },
	  -- if you want to build from source then do `make BUILD_FROM_SOURCE=true`
	  build = "make",
	  -- build = "powershell -ExecutionPolicy Bypass -File Build.ps1 -BuildFromSource false" -- for windows
	  dependencies = {
	    "nvim-treesitter/nvim-treesitter",
	    "stevearc/dressing.nvim",
	    "nvim-lua/plenary.nvim",
	    "MunifTanjim/nui.nvim",
	    "echasnovski/mini.pick", -- for file_selector provider mini.pick
	    "nvim-telescope/telescope.nvim", -- for file_selector provider telescope
	    "hrsh7th/nvim-cmp", -- autocompletion for avante commands and mentions
	    "ibhagwan/fzf-lua", -- for file_selector provider fzf
	    "nvim-tree/nvim-web-devicons", -- or echasnovski/mini.icons
	    "zbirenbaum/copilot.lua", -- for providers='copilot'
	    {
	      -- support for image pasting
	      "HakonHarnes/img-clip.nvim",
	      event = "VeryLazy",
	      opts = {
		-- recommended settings
		default = {
		  embed_image_as_base64 = false,
		  prompt_for_file_name = false,
		  drag_and_drop = {
		    insert_mode = true,
		  },
		  -- required for Windows users
		  use_absolute_path = true,
		},
	      },
	    },
	    {
	      -- Make sure to set this up properly if you have lazy=true
	      'MeanderingProgrammer/render-markdown.nvim',
	      opts = {
		file_types = { "markdown", "Avante" },
		windows = {
			position = 'right',
			width = 100
		}
	      },
	      ft = { "markdown", "Avante" },
	    },
	  },
	}
}
