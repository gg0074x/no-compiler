vim.filetype.add({ extension = { nc = "nc" } })

vim.api.nvim_create_autocmd("FileType", {
  pattern = "nc",
  callback = function(event) vim.bo[event.buf].commentstring = "# %s" end,
})

require("nvim-treesitter.parsers").get_parser_configs().nc = {
  install_info = {
    url = "https://github.com/gg0074x/no-compiler",
    files = { "tree-sitter-nc/src/parser.c" },
    branch = "master",
  },
  filetype = "nc",
}
