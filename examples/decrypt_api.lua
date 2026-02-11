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
        key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
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

      ---------
      -- api 03
      ---------
      local enc = "-----BEGIN AGE ENCRYPTED FILE-----\nYWdlLWVuY3J5cHRpb24ub3JnL3YxCi0+IFgyNTUxOSA0MTJ6eFpNSkJzWWZQOGhp\nK3MzZi9KMDhYY0M4azJmNkhhWVlzbHRKRlI0CnF1a2E4UHdKM3NtWHZmRXpKZ1l2\nRTBnUDdwR3JFK0M0YTR5UFA4bENXUnMKLT4gQ3YhLEtELHgtZ3JlYXNlICtAe0Bl\nCk1KQVZBY3Q0NWhPZHFyQQotLS0gaC9IajVmaVB2Mm9BcHQ4RlFtb2dDV3BXSm1G\nUVlUTnp3bG11VmMxQ2xCbwpnuz8DboqAagJFzdzzH1Rw+CmXqA/bdcXf3vKE3mz8\nNjRbL4GsObbj8IomB27BmA5vew==\n-----END AGE ENCRYPTED FILE-----"

      local secret_03 = age.decrypt_from_string(enc)

      print(secret_03)
    end,
  },
}
