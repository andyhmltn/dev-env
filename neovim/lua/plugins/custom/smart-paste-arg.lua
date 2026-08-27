local function smart_paste_arg()
    local line = vim.api.nvim_get_current_line()
    local cursor_col = vim.api.nvim_win_get_cursor(0)[2]

    -- Check if there's a comma before and after the cursor in this line
    local has_comma_before = line:sub(1, cursor_col):find(",") ~= nil
    local has_comma_after = line:sub(cursor_col + 1):find(",") ~= nil

    -- Check if next character is a comma
    local next_char_is_comma = line:sub(cursor_col + 2, cursor_col + 2) == ","

    -- Count commas in the current line
    local comma_count = 0
    for _ in line:gmatch(",") do
        comma_count = comma_count + 1
    end

    -- Check if we're at the beginning of a list (cursor is before the first comma)
    local first_comma_pos = line:find(",")
    local at_list_start = first_comma_pos and cursor_col < first_comma_pos

    -- Decide which paste command to use
    if comma_count <= 1 then
        -- Only one or zero commas on this line - use multiline paste (-pl)
        vim.cmd("normal o")
        vim.cmd("normal pA,")
    elseif at_list_start then
        -- At the start of a list - use paste before (-pb)
        vim.cmd("normal ebPa, ")
        vim.cmd("normal :w<Cr>")
    elseif next_char_is_comma then
        -- Next character is a comma - use paste without moving to end (-pe)
        vim.cmd("normal a, ")
        vim.cmd("normal p")
    elseif has_comma_before and has_comma_after then
        -- In middle of list - use paste after end of word (-pa)
        vim.cmd("normal ea, ")
        vim.cmd("normal p")
    else
        -- Default to paste after
        vim.cmd("normal ea, ")
        vim.cmd("normal p")
    end
end

vim.api.nvim_create_user_command("SmartPasteArg", smart_paste_arg, {})
vim.keymap.set("n", "<Leader>u", smart_paste_arg, {noremap = false})

-- List arguments
-- Delete list argument
vim.keymap.set("n", "<Leader>da", "ebdf :w<Cr>", {noremap = false})
-- " Pastes the current clipboard item as an argument with a comma before. EG
-- " when copying 'test' and using this at the end of hello, world you will
-- " get hello, world, test
vim.keymap.set(
    "n",
    "<Leader>-pa",
    function()
        vim.cmd("normal ea, ")
        vim.cmd("normal p")
    end,
    {noremap = false}
)
-- " same as above, but does not go to the end of the word
vim.keymap.set(
    "n",
    "<Leader>-pe",
    function()
        vim.cmd("normal a, ")
        vim.cmd("normal p")
    end,
    {noremap = false}
)
-- " pastes an argument before the current parameter
vim.keymap.set(
    "n",
    "<Leader>-pb",
    function()
        vim.cmd("normal ebPa, ")
        vim.cmd("normal :w<Cr>")
    end,
    {noremap = false}
)
-- " Paste as list so add a new line above current and paste with a comma at the
-- " end
vim.keymap.set(
    "n",
    "<Leader>-pl",
    function()
        vim.cmd("normal o")
        vim.cmd("normal pA,")
    end,
    {noremap = false}
)

