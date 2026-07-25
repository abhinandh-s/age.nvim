
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


--[[
vim.pack.add({
  'https://github.com/abhinandh-s/age.nvim',
})


require('age').setup({
  encrypt_and_del = true,
  key_file = vim.fn.expand("tests/test_keys.txt"),
})
]]--