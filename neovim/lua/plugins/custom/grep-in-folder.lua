

local telescope = require("telescope")
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")
local builtin = require("telescope.builtin")
local themes = require("telescope.themes")

-- Function to search in a specific folder
function live_grep_in_folder()
    -- Select a folder first
    require("telescope.builtin").find_files(
        {
            prompt_title = "Select Directory",
            -- find_command = { "find", ".", "-type", "d", "-maxdepth", "2" },  -- List directories only
            cwd = vim.fn.getcwd(),
            find_command = {"find", ".", "-type", "d"},
            attach_mappings = function(prompt_bufnr, map)
                local function set_directory()
                    local selection = action_state.get_selected_entry()
                    local dir = selection[1] -- Selected directory

                    vim.g.selected_dir = dir

                    -- Close the current picker and call live_grep on the selected directory
                    actions.close(prompt_bufnr)

                    -- Call live_grep on the selected directory
                    require("telescope.builtin").live_grep(
                        {
                            search_dirs = {dir},
                            prompt_title = "Live Grep in " .. dir
                        }
                    )
                end

                -- Map selection to `set_directory` function
                map("i", "<CR>", set_directory)
                map("n", "<CR>", set_directory)

                return true
            end
        }
    )
end

function live_grep_previous_folder()
    print(vim.g.selected_dir)
    require("telescope.builtin").live_grep(
        {
            search_dirs = {vim.g.selected_dir}
        }
    )
end

vim.api.nvim_set_keymap("n", "<leader>fg", ":lua live_grep_in_folder()<CR>", {noremap = true, silent = true})
vim.api.nvim_set_keymap("n", "<leader>-fg", ":lua live_grep_previous_folder()<CR>", {noremap = true, silent = true})


