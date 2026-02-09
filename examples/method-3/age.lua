local plugins = {}

table.insert(plugins, {
  "abhinandh-s/age.nvim",
  cmd = { "Age" },
  config = function()
    require('age').setup({
      encrypt_and_del = true,
      public_key = "age1jmcmqq4jpzcutzeajxavlapee57rhp3gzprlc5se8vvsk9ej3ezq7nh94p",
      key_file = vim.fn.expand("~/.local/share/age/key.txt"), -- no need to specify `private_key` now 
    })
  end
})

return plugins
