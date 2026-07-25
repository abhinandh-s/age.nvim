vim.pack.add({
  'https://github.com/abhinandh-s/age.nvim',
})


require('age').setup({
  encrypt_and_del = true,
  key_file = vim.fn.expand("test/keys.txt"),
})