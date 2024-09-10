-- Tree-Sitter
local has_treesitter, treesitter = pcall(require, "nvim-treesitter.parsers")

if has_treesitter then
  vim.filetype.add({ extension = { nc = "nc" } })

  vim.api.nvim_create_autocmd("FileType", {
    pattern = "nc",
    callback = function(event) vim.bo[event.buf].commentstring = "# %s" end,
  })

  treesitter.get_parser_configs().nc = {
    install_info = {
      url = "https://github.com/gg0074x/no-compiler",
      files = { "tree-sitter-nc/src/parser.c" },
      branch = "master",
    },
    filetype = "nc",
  }
end

-- Lsp Config

local has_lspconfig, configs = pcall(require, "lspconfig.configs")

if has_lspconfig then
  local util = require 'lspconfig.util'

  if not configs.nc then
    configs.nc = {
      default_config = {
        cmd = { 'nc', 'lsp' },
        filetypes = { 'nc' },
        root_dir = util.find_git_ancestor,
        single_file_support = true,
      },
      docs = {
        description = [[
    https://github.com/gg0074x/no-compiler

    No-Compiler built-in language server.
    ]],
        default_config = {
          root_dir = [[util.find_git_ancestor]],
        },
      },
    }
  end
end
