-- ~/.config/nvim/lua/plugins/age.lua

return {
    'abhi-xyz/age.nvim',
    cmd = { "Age" },
    config = function()
      local key = require('key')

      require('age').setup({
        encrypt_and_del = true, -- will remove the original file after encrypting.
        public_key = "ageXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        private_key = key.private_key, -- defined in another lua file which is not included in git for safety
      })
    end
}
