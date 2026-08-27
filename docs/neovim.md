# Neovim Configuration

A Lua-based Neovim configuration using Lazy.nvim for plugin management. Optimized for TypeScript, Go, and Lua development.

## Leader Key

The leader key is **Space**.

## Core Settings

| Setting | Value |
|---------|-------|
| Line numbers | Relative |
| Search | Case-insensitive, smart case |
| Clipboard | System clipboard (`unnamedplus`) |
| Swap files | Disabled |
| Color scheme | Catppuccin Macchiato (transparent) |
| Font | Iosevka Nerd Font (GUI) |

## File Navigation

| Keybinding | Action |
|------------|--------|
| `<Space><Tab>` | Git files (Telescope) |
| `<Space>'` | Live grep (Telescope) |
| `<Space>"` | Resume last Telescope search |
| `<Space>fb` | Open buffers |
| `<Space>ff` | Find files in current buffer's directory |
| `<Space>l` | Toggle file tree (NvimTree) |
| `<Ctrl-a>` | Previous buffer (BufSurf) |
| `<Ctrl-f>` | Next buffer (BufSurf) |

## Code Navigation

| Keybinding | Action |
|------------|--------|
| `gd` | Go to definition |
| `gr` | Find references (Telescope) |
| `gi` | Go to implementation |
| `K` | Hover documentation |
| `<Space>t` | Jump to LSP symbol (Namu) |
| `]g` | Next diagnostic |
| `[g` | Previous diagnostic |
| `<Space>e` | Open diagnostic float |

## Code Actions

| Keybinding | Action |
|------------|--------|
| `<Space>rn` | Rename symbol |
| `<Space>ca` | Code actions |
| `<Space>i` | Auto-import missing imports (TS) |
| `:Format` | Format file via LSP |
| `:Prettier` | Format with Prettier |

## Motion (Flash)

| Keybinding | Action |
|------------|--------|
| `s` | Flash jump (case-insensitive search) |
| `S` | Flash treesitter selection |
| `r` (operator mode) | Remote flash |
| `R` | Treesitter search |
| `gel` | Jump to empty line |

## Window Management

| Keybinding | Action |
|------------|--------|
| `<Space>W` | Vertical split |
| `<Space>V` | Horizontal split |
| `<Ctrl-h/j/k/l>` | Navigate between panes (works with tmux) |

## Search and Replace

| Keybinding | Action |
|------------|--------|
| `<Space>S` | Open Spectre (search/replace) |
| `<Space>gg` | Open Lazygit in new tmux window |

## Completion

Completion is handled by nvim-cmp with LuaSnip:

| Key | Action |
|-----|--------|
| `<Tab>` | Next completion item / expand snippet |
| `<Shift-Tab>` | Previous completion item |
| `<Up>/<Down>` | Navigate completion menu |
| `<Enter>` | Confirm selection |
| `<Ctrl-n>` | Jump to next snippet placeholder |
| `<Ctrl-p>` | Jump to previous snippet placeholder |

Sources: LSP, snippets, buffer, path.

## Text Objects

| Key | Object |
|-----|--------|
| `ii`, `ai` | Indent level |
| `cs`, `ds`, `ys` | Surround (vim-surround) |

## Case Conversion (vim-camelsnek)

Convert word under cursor between case styles:
- `:Snek` - snake_case
- `:Camel` - camelCase
- `:Kebab` - kebab-case

## LSP Servers

Configured via Mason:

| Language | Server |
|----------|--------|
| Go | gopls (auto-format on save) |
| Lua | lua_ls |
| TypeScript | typescript-tools.nvim |

## Formatting

Handled by conform.nvim:

| Filetype | Formatter |
|----------|-----------|
| JavaScript/TypeScript | prettierd |
| JSON | prettierd |
| Markdown | prettierd |
| Go | gopls (on save) |

## Plugins

Major plugins included:

- **Telescope** - Fuzzy finder
- **NvimTree** - File explorer (right side)
- **Treesitter** - Syntax highlighting
- **Flash** - Motion plugin
- **Spectre** - Search and replace
- **Lazygit** - Git UI integration
- **claude.vim** - AI assistant
- **Noice** - UI improvements
- **Mini** - Starter screen, pairs
- **VimWiki** - Note-taking

## Vimwiki

Personal wiki stored in `~/vimwiki/` using markdown:

| Keybinding | Action |
|------------|--------|
| `<Ctrl-t>` | Open wiki index in split |
| `<Space>-otd` | Open wiki index in split |

## Claude Integration

| Keybinding | Action |
|------------|--------|
| `<Space>cs` | Send chat message |
| `<Space>ci` | Implement |
| `<Space>cx` | Cancel response |
| `<Ctrl-]>` | Send chat message |

## Custom Registers

| Register | Content |
|----------|---------|
| `"t` | Prepends "type " |
| `"p` | Appends ", " |

## Custom Plugins

Located in `lua/plugins/custom/`:

- `grep-in-folder` - Grep within specific folder
- `smart-paste-arg` - Smart argument pasting
- `wildcard-search` - Wildcard pattern search
- `eslint` - ESLint integration

## Editing Behavior

- `d` and `c` use black hole register (no yank on delete/change)
- Paste in visual mode preserves register
- Search results auto-center (`n`, `N`, `*`, `{`, `}`)
