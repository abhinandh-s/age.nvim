local plugins = {}

local public_key = os.getenv("AGE_PUBLIC_KEY")
local private_key = os.getenv("AGE_PRIVATE_KEY")

if not public_key or not private_key then
  vim.notify("AGE keys are not set in environment variables!", vim.log.levels.ERROR)
  return
else
    table.insert(plugins, {
      "abhinandh-s/age.nvim",
      cmd = { "Age" },
      config = function()
        require('age').setup({
          encrypt_and_del = true,
          public_key = public_key,
          private_key = private_key,
        })
      end
    })
end

return plugins
