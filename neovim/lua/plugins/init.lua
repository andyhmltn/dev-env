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
				ensure_installed = { "ts-lua", "gopls" },
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
				-- Optional: Format on save, or define a command
				vim.api.nvim_create_user_command("Format", function()
					vim.lsp.buf.format()
				end, {})
			end,

			settings = {
			},
		}
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
							cmp.select_next_item()
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
					-- { name = "luasnip" },
					{ name = "buffer" },
					{ name = "path" },
				}),
			})
		end
	},
}
