vim.keymap.set(
    {"n", "o", "x"},
    "s",
    function()
        local gi = vim.go.ignorecase
        local gs = vim.go.smartcase
        vim.go.ignorecase = true
        vim.go.smartcase = true
        require("flash").jump()
        vim.go.ignorecase = gi
        vim.go.smartcase = gs
    end,
    {desc = "Flash"}
)

vim.keymap.set(
    {"n", "o", "x"},
    "S",
    function()
        require("flash").treesitter()
    end,
    {desc = "Flash treesitter"}
)
vim.keymap.set(
    {"o"},
    "r",
    function()
        require("flash").remote()
    end,
    {desc = "Remote flash"}
)
vim.keymap.set(
    {"n", "o"},
    "R",
    function()
        require("flash").treesitter_search()
    end,
    {desc = "Treesitter search"}
)

vim.keymap.set(
    {"n", "o", "x"},
    "gel",
    function()
        require("flash").jump(
            {
                search = {mode = "search", max_length = 0},
                label = {after = {0, 0}},
                pattern = "^$"
            }
        )
    end,
    {desc = "Jump to line"}
)
