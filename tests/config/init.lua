local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({
      { "Failed to clone lazy.nvim:\n", "ErrorMsg" },
      { out, "WarningMsg" },
      { "\nPress any key to exit..." },
    }, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

require("lazy").setup({
  spec = {
    {
      'abhinandh-s/age.nvim',
        build = [[
          echo "Building age.nvim from source..."
          cargo build --release --target-dir ./target
          mkdir -p lua
          mv target/release/libage.so lua/age.so
          echo "Build Completed."
        ]],
        cmd = { "Age" },
        config = function()
          require('age').setup({
            encrypt_and_del = true,
            key_file = vim.fn.expand("key.txt"),
          })
        end
      }
   },
})

local success, err = pcall(function()
  vim.cmd("Age genkey")
  print("\n[INFO]: Key generated.")
end)

if not success then
  print("\n[CMD Error]: " .. tostring(err))
  vim.cmd("cquit")
end

vim.wait(1000)

print(vim.fn.getcwd())

local success, err = pcall(function()
  vim.cmd("e secret.md")
  vim.cmd("Age encrypt")
  print("\n[INFO]: File encrypted.")
end)

if not success then
  print("\n[CMD Error]: " .. tostring(err))
  vim.cmd("cquit")
end

vim.wait(1000)

local success, err = pcall(function()
  vim.cmd("e secret.md.age")
  vim.cmd("Age decrypt")
  print("\n[INFO]: File decrypted.")
end)

if not success then
  print("\n[CMD Error]: " .. tostring(err))
  vim.cmd("cquit")
end



      local age = require("age")

      ---------
      -- api 01
      ---------

      -- Load the secret
      local secret = age.decrypt_to_string(vim.fn.expand("secret.md.age"))

      print(secret)

      ---------
      -- api 02
      ---------
      local secret_02 = age.decrypt_to_string_with_identities(
        vim.fn.expand("secret.md.age"),
        {
          vim.fn.expand("key.txt"),
        }
      )

      print(secret_02)

      ---------
      -- api 03
      ---------
   --   local enc = "-----BEGIN AGE ENCRYPTED FILE-----\nYWdlLWVuY3J5cHRpb24ub3JnL3YxCi0+IFgyNTUxOSA0MTJ6eFpNSkJzWWZQOGhp\nK3MzZi9KMDhYY0M4azJmNkhhWVlzbHRKRlI0CnF1a2E4UHdKM3NtWHZmRXpKZ1l2\nRTBnUDdwR3JFK0M0YTR5UFA4bENXUnMKLT4gQ3YhLEtELHgtZ3JlYXNlICtAe0Bl\nCk1KQVZBY3Q0NWhPZHFyQQotLS0gaC9IajVmaVB2Mm9BcHQ4RlFtb2dDV3BXSm1G\nUVlUTnp3bG11VmMxQ2xCbwpnuz8DboqAagJFzdzzH1Rw+CmXqA/bdcXf3vKE3mz8\nNjRbL4GsObbj8IomB27BmA5vew==\n-----END AGE ENCRYPTED FILE-----"

   --   local secret_03 = age.decrypt_from_string(enc)

   --   print(secret_03)