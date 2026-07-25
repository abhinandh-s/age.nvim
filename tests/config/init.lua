
-- Bootstrap lazy.nvim
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

-- Make sure to setup `mapleader` and `maplocalleader` before
-- loading lazy.nvim so that mappings are correct.
-- This is also a good place to setup other settings (vim.opt)
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- Setup lazy.nvim
require("lazy").setup({
  spec = {
    {
    'abhinandh-s/age.nvim',
    cmd = { "Age" },
    config = function()
      require('age').setup({
        key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
      })
    end
}
  },
  -- Configure any other settings here. See the documentation for more details.
  -- colorscheme that will be used when installing plugins.
  install = { colorscheme = { "habamax" } },
  -- automatically check for plugin updates
  checker = { enabled = true },
})


local success, err = pcall(function()
  vim.cmd("Age genkey")
  print("[INFO]: Key generated.")
end)

if not success then
  print("[CMD Error]: " .. tostring(err))
  vim.cmd("cquit")
end


--[[
vim.pack.add({
  'https://github.com/abhinandh-s/age.nvim',
})


require('age').setup({
  encrypt_and_del = true,
  key_file = vim.fn.expand("tests/test_keys.txt"),
})
]]--