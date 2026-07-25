require("lazy")

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