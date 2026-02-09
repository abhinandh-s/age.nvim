return {
  {
    "folke/tokyonight.nvim",
    dependencies = {
      'abhinandh-s/age.nvim' -- # add age as dependency
    },
    config = function()
      local age = require("age")

      ---------
      -- api 01 
      ---------
      age.setup({
          private_key = "private_key",
        })

      -- Load the secret
      local secret = age.decrypt_to_string(vim.fn.expand("~/.config/nvim/top_secret.txt.age"))

      print(secret)

      ---------
      -- api 02 
      ---------
      local secret_02 = age.decrypt_to_string_with_identities(
        vim.fn.expand("~/.config/nvim/top_secret.txt.age"),
        {
          vim.fn.expand("~/.local/share/age/key.txt"),
        }
      )

      print(secret_02)
    end,
  },
}
