<div align="center">

# just.nvim

</div>

<div align="center">

**Neovim plugin for encrypting and decrypting text files inside neovim using `age` with ease.**

</div>

## Installation

Install Just using your favorite plugin manager. For example, with [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
-- ~/.config/nvim/lua/plugins/just.lua

{
    'abhi-xyz/just.nvim',
    cmd = { "Just", "J" },
    config = function()
      local key = require('key')

      require('just').setup({
        public_key = "ageXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
        private_key = key.private_key, -- defined in another lua file which is not included in git for safety
      })
    end
}
```
```lua
-- ~/.config/nvim/lua/key.lua

return {
  private_key = "AGE-SECRET-KEY-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
}
```

## Usage

Just provides the `:Just` command with the following syntax:

```vim
:Just [action]
```

- `[action]` can be one of:
  - `encrypt`,
  - `decrypt`,
  - `genkey`

#### Examples:

- Generates an age key pair into key.txt in current working directory.

```vim
:Just genkey
```

- Kills the current buffer and switches to a previous buffer or creates a scratch buffer in case there is no buffer to switch, then encrypts the file with the provided age key.

```vim
:Just encrypt
```

- Decrypts the currently opened encrypted file, and switches to the decrypted file. 
```vim
:Just decrypt
```
