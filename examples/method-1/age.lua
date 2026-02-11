local plugins = {}

table.insert(plugins, {
  "abhinandh-s/age.nvim",
  cmd = { "Age" },
  config = function()
    require('age').setup({
      encrypt_and_del = true,
      key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
    })
  end
})

return plugins
