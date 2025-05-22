-- lua/frontend_lint_qf.lua  ───────────────────────────────────────────────
local M = {}

-- Command: run your existing npm script *inside* ./frontend
local MAKEPRG = table.concat({
  "npm", "run", "--silent",         -- no npm banner
  "--prefix", "frontend",           -- cd into ./frontend first
  "check:lint",                     -- ← the script you already trust
  "--",                             -- everything after here goes to ESLint
  "--no-color", "-f", "unix"        -- plain text + unix formatter
}, " ")

-- ESLint + -f unix prints:  path/to/file:line:col: message
local ERRORFORMAT = "%f:%l:%c: %m"

--- Run the linter synchronously and open quickfix
function M.run()
  -- buffer-local so we don’t touch other projects
  vim.opt_local.makeprg     = MAKEPRG
  vim.opt_local.errorformat = ERRORFORMAT
  vim.cmd("silent make")    -- populate the quickfix list
  vim.cmd("copen")          -- show it
end

function M.setup()
  vim.keymap.set(
    "n", "<leader>el",
    M.run,
    { desc = "Lint ./frontend → quickfix" }
  )
end

return M
