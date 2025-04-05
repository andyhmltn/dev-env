local builtin = require("telescope.builtin")
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")

local searches = {
    {label = "Graph schema", prefix = "graph", suffix = "schema"},
    {label = "Graph resolvers", prefix = "graph", suffix = "resolvers"},
    {label = "Repos", prefix = "repos", suffix = "", doNotAutoSelect = true},
    {label = "Services", prefix = "services", suffix = "", doNotAutoSelect = true},
    {label = "Protos", prefix = "protos", suffix = "", doNotAutoSelect = true},
    {label = "Datasources", prefix = "datasources", suffix = ""},
    {label = "Transport Grpc", prefix = "", suffix = "transportgrpc"},
    {label = "Transport Kafka", prefix = "", suffix = "transportkafka"}
}

local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local scandir = require("plenary.scandir")

function volcano_dirs(callback)
    local dir_path = "packages/apps"

    -- Get subdirectories (just one depth)
    local subdirs =
        scandir.scan_dir(
        dir_path,
        {
            only_dirs = true,
            depth = 1
        }
    )

    -- Extract the final folder name for each subdir
    local entries = {}
    for _, path in ipairs(subdirs) do
        table.insert(entries, vim.fn.fnamemodify(path, ":t"))
    end

    pickers.new(
        {},
        {
            prompt_title = "Subdirectories of " .. dir_path,
            finder = finders.new_table(
                {
                    results = entries
                }
            ),
            sorter = conf.generic_sorter({}),
            attach_mappings = function(prompt_bufnr, map)
                actions.select_default:replace(
                    function()
                        -- Get the currently "selected" entry (could be nil)
                        local selection = action_state.get_selected_entry()
                        -- Get whatever the user typed in the prompt
                        local user_input = action_state.get_current_line()

                        -- Close Telescope
                        actions.close(prompt_bufnr)

                        -- If the user picked an actual entry, use that;
                        -- otherwise, fallback to whatever is typed in the prompt.
                        if selection then
                            callback(selection.value)
                        else
                            callback(user_input)
                        end
                    end
                )
                return true
            end
        }
    ):find()
end

function wildcard_search(prefix, suffix, doNotAutoSelect)
    local pickers = require("telescope.pickers")
    local finders = require("telescope.finders")
    local actions = require("telescope.actions")
    local action_state = require("telescope.actions.state")
    local builtin = require("telescope.builtin")
    local conf = require("telescope.config").values

    volcano_dirs(
        function(selected)
            local pattern = prefix .. selected .. suffix
            builtin.git_files(
                {
                    prompt_title = "Git Files (" .. pattern .. ")",
                    initial_mode = doNotAutoSelect and "normal" or insert,
                    attach_mappings = function(git_bufnr, _)
                        local picker2 = action_state.get_current_picker(git_bufnr)
                        if picker2 and picker2.set_prompt then
                            picker2:set_prompt(pattern)
                        end

                        -- Automatically move selection onto the first result once it appears
                        -- We defer a little so Telescope has time to load results before we
                        -- move the selection. Without defer, there might be no results yet.
                        if not doNotAutoSelect then
                            vim.defer_fn(
                                function()
                                    actions.select_default(git_bufnr)
                                end,
                                100
                            )
                        end

                        return true
                    end
                }
            )
        end
    )
end

function pick_wildcard_search()
    local pickers = require("telescope.pickers")
    local finders = require("telescope.finders")
    local actions = require("telescope.actions")
    local action_state = require("telescope.actions.state")
    local conf = require("telescope.config").values

    pickers.new(
        {},
        {
            prompt_title = "Saved Searches",
            finder = finders.new_table {
                results = searches,
                entry_maker = function(entry)
                    return {
                        display = entry.label,
                        ordinal = entry.label,
                        value = entry
                    }
                end
            },
            sorter = conf.generic_sorter({}),
            attach_mappings = function(prompt_bufnr, map)
                -- On <CR>, close the picker and run git_files with the chosen pattern
                local on_select = function()
                    local selection = action_state.get_selected_entry()
                    actions.close(prompt_bufnr)
                    local chosen = selection.value
                    wildcard_search(chosen.prefix, chosen.suffix, chosen.doNotAutoSelect)
                end

                map("i", "<CR>", on_select)
                map("n", "<CR>", on_select)

                return true
            end
        }
    ):find()
end

vim.api.nvim_create_user_command("WildcardSearch", wildcard_search, {})
vim.keymap.set("n", "<leader>-tt", pick_wildcard_search)

