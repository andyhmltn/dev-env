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
	},

	-- Themes
	{ "folke/tokyonight.nvim" },

	{
		"folke/noice.nvim",
		event = "VeryLazy",
		opts = {

		},
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
				},

				-- make EVERY vim.notify() use nvim-notify (Noice piggy-backs on that)
				config = function(_, opts)
				  local notify = require("notify")
				  notify.setup(opts)
				  vim.notify = notify
				end,
		      },
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
			-- Set up LSP keymaps via LspAttach autocmd (Neovim 0.11+ recommended approach)
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

			-- Configure gopls using vim.lsp.config (Neovim 0.11+)
			vim.lsp.config.gopls = {
				cmd = { "gopls" },
				filetypes = { "go", "gomod", "gowork", "gotmpl" },
				root_markers = { "go.work", "go.mod", ".git" },
				settings = {},
			}

			-- Format Go files on save
			vim.api.nvim_create_autocmd("BufWritePre", {
				pattern = { "*.go" },
				callback = function(args)
					vim.lsp.buf.format({ bufnr = args.buf })
				end,
			})

			-- Configure lua_ls using vim.lsp.config (Neovim 0.11+)
			vim.lsp.config.lua_ls = {
				cmd = { "lua-language-server" },
				filetypes = { "lua" },
				root_markers = { ".luarc.json", ".luarc.jsonc", ".luacheckrc", ".stylua.toml", "stylua.toml", "selene.toml", "selene.yml", ".git" },
				settings = {
					Lua = {
						diagnostics = { globals = { "vim" } },
						workspace = { checkThirdParty = false },
						telemetry = { enable = false },
					}
				}
			}

			-- Enable the configured LSP servers
			vim.lsp.enable({ "gopls", "lua_ls" })
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
				vim.keymap.set("n", "gr", function() require('telescope.builtin').lsp_references() end, bufopts)
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
		opts = function()
			local util = require("conform.util")
			return {
				format_on_save = function(bufnr)
					local bufname = vim.api.nvim_buf_get_name(bufnr)
					local dir = vim.fn.fnamemodify(bufname, ":h")
					local prettier_config = vim.fs.find(
						{ ".prettierrc", ".prettierrc.js", ".prettierrc.json", "prettier.config.js" },
						{ path = dir, upward = true, type = "file" }
					)
					if #prettier_config > 0 then
						return {
							timeout_ms = 2000,
							lsp_fallback = false,
							formatters = { "prettierd" },
						}
					end
					local biome_config = vim.fs.find("biome.json", { path = dir, upward = true, type = "file" })
					if #biome_config > 0 then
						return {
							timeout_ms = 2000,
							lsp_fallback = false,
							formatters = { "biome" },
						}
					end
					return {
						timeout_ms = 2000,
						lsp_fallback = false,
					}
				end,
				formatters = {
					prettierd = {
						command = "prettierd",
						stdin = true,
						cwd = util.root_file({
							".prettierrc",
							".prettierrc.js",
							".prettierrc.json",
							"prettier.config.js",
							"package.json",
						}),
					},
				},
				formatters_by_ft = {
					javascript      = { "prettierd" },
					javascriptreact = { "prettierd" },
					typescript      = { "prettierd" },
					typescriptreact = { "prettierd" },
					json            = { "prettierd" },
					markdown        = { "prettierd" },
				},
			}
		end,
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
			"rafamadriz/friendly-snippets", -- Collection of snippets
		},
		config = function()
			local cmp = require("cmp")
			local luasnip = require("luasnip")
			
			-- Configure LuaSnip
			luasnip.config.set_config({
				history = true,
				updateevents = "TextChanged,TextChangedI",
				enable_autosnippets = true,
			})
			
			-- Load custom snippets
			require("luasnip-config")
			
			-- Also try to load VSCode-style snippets
			require("luasnip.loaders.from_vscode").lazy_load({
				paths = { vim.fn.stdpath("config") .. "/snippets" }
			})
			
			-- Snippet keymaps
			vim.keymap.set({"i", "s"}, "<C-n>", function()
				if luasnip.expand_or_jumpable() then
					luasnip.expand_or_jump()
				end
			end, { silent = true })
			
			vim.keymap.set({"i", "s"}, "<C-p>", function()
				if luasnip.jumpable(-1) then
					luasnip.jump(-1)
				end
			end, { silent = true })

			cmp.setup({
				snippet = {
					expand = function(args)
						luasnip.lsp_expand(args.body)
					end,
				},
				confirmation = { completeopt = 'menu,menuone,noinsert' },
				formatting = {
					format = function(entry, vim_item)
						vim_item.menu = ({
							nvim_lsp = "[LSP]",
							luasnip = "[Snippet]",
							buffer = "[Buffer]",
							path = "[Path]",
						})[entry.source.name]
						return vim_item
					end,
				},
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
		end
	},
	{
	  "afewyards/codereview.nvim",
	  dependencies = { "nvim-lua/plenary.nvim" },
	  cmd = {
	    "CodeReview",
	    "CodeReviewAI",
	    "CodeReviewAIFile",
	    "CodeReviewStart",
	    "CodeReviewSubmit",
	    "CodeReviewApprove",
	    "CodeReviewOpen",
	    "CodeReviewPipeline",
	    "CodeReviewComments",
	    "CodeReviewFiles",
	    "CodeReviewToggleScroll",
	    "CodeReviewCommits",
	  },
	  opts = {},
	}
}
