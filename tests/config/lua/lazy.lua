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
