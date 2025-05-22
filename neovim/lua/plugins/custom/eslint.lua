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

--- Run the linter asynchronously and open quickfix when done
function M.run()
  -- buffer-local so we don't touch other projects
  vim.opt_local.makeprg = MAKEPRG
  vim.opt_local.errorformat = ERRORFORMAT
  
  -- Show a notification that linting is in progress
  local has_noice, noice = pcall(require, "noice")
  if has_noice then
    noice.notify("Frontend linting in progress...", "info", {
      title = "ESLint",
      timeout = 3000,
    })
  else
    vim.notify("Frontend linting in progress...", vim.log.levels.INFO)
  end
  
  -- Run the linting asynchronously using vim.fn.jobstart
  local cmd = vim.o.shell .. " -c '" .. MAKEPRG .. "'"
  
  -- Start the job
  local output = {}
  local job_id = vim.fn.jobstart(cmd, {
    on_stdout = function(_, data)
      if data then
        for _, line in ipairs(data) do
          if line ~= "" then
            table.insert(output, line)
          end
        end
      end
    end,
    on_stderr = function(_, data)
      if data then
        for _, line in ipairs(data) do
          if line ~= "" then
            table.insert(output, line)
          end
        end
      end
    end,
    on_exit = function(_, exit_code)
      vim.schedule(function()
        -- Clear the current quickfix list
        vim.fn.setqflist({}, 'r')
        
        -- Parse the output using errorformat
        local qf_entries = {}
        for _, line in ipairs(output) do
          local filename, lnum, col, text = string.match(line, "([^:]+):(%d+):(%d+): (.*)")
          if filename and lnum and col and text then
            table.insert(qf_entries, {
              filename = filename,
              lnum = tonumber(lnum),
              col = tonumber(col),
              text = text,
            })
          end
        end
        
        -- Set the quickfix list with the parsed entries
        vim.fn.setqflist(qf_entries, 'r')
        
        -- Show a completion notification
        if has_noice then
          noice.notify("Frontend linting complete", "success", {
            title = "ESLint",
            timeout = 3000,
          })
        else
          vim.notify("Frontend linting complete", vim.log.levels.INFO)
        end
        
        -- Open the quickfix window if there are errors
        if #qf_entries > 0 then
          vim.cmd("copen")
        else
          vim.notify("No linting errors found", vim.log.levels.INFO)
        end
      end)
    end,
    stdout_buffered = false,
    stderr_buffered = false,
  })
  
  if job_id <= 0 then
    vim.notify("Failed to start linting job", vim.log.levels.ERROR)
  end
end

function M.setup()
  vim.keymap.set(
    "n", "<leader>el",
    M.run,
    { desc = "Lint ./frontend → quickfix (async)" }
  )
end

return M
