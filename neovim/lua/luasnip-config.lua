local ls = require("luasnip")
local s = ls.snippet
local t = ls.text_node
local i = ls.insert_node
local f = ls.function_node
local d = ls.dynamic_node
local sn = ls.snippet_node

-- Helper to repeat nodes
local function rep(idx)
  return f(function(args) return args[1] end, {idx})
end

-- TypeScript/React snippets
ls.add_snippets("typescriptreact", {
  s("rfc", {
    t("import React, { FC } from \"react\";"),
    t({"", "", "interface "}),
    i(1, "ComponentName"),
    t("Props {"),
    t({"", "  "}),
    i(2),
    t({"", "}", "", "export const "}),
    rep(1),
    t(": FC<"),
    rep(1),
    t("Props> = ({"),
    t({"", "  "}),
    i(3),
    t({"", "}) => {", "  return () "}),
    i(4),
    t({";", "};"}),
  }),
})

-- Also add to typescript files
ls.add_snippets("typescript", {
  s("rfc", {
    t("import React, { FC } from \"react\";"),
    t({"", "", "interface "}),
    i(1, "ComponentName"),
    t("Props {"),
    t({"", "  "}),
    i(2),
    t({"", "}", "", "export const "}),
    rep(1),
    t(": FC<"),
    rep(1),
    t("Props> = ({"),
    t({"", "  "}),
    i(3),
    t({"", "}) => {", "  return () "}),
    i(4),
    t({";", "};"}),
  }),
})
