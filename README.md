<div align="center">

# age.nvim

</div>

<div align="center">

**Neovim plugin for encrypting and decrypting text files inside neovim using [age](https://github.com/FiloSottile/age) with ease.**

</div>

## Table of Contents  

- [Installation](#installation)  
- [Usage](#usage)  
- [What is age?](#what-is-age)  

## Installation

> [!WARNING]
> Master branch supports Neovim version >= 0.11

```lua
  ...  
  'abhinandh-s/age.nvim',
  branch = "nvim-v10", -- pin branch for Neovim version 0.10
  ...
```

Install Age using your favorite plugin manager. For example, with [lazy.nvim](https://github.com/folke/lazy.nvim):

### Dependencies

**None**. `age.nvim` does not require the `age` CLI to be installed separately.
Everything needed for encryption and decryption is handled within the plugin itself.


### config via env variable

```lua
-- ~/.config/nvim/lua/plugins/age.lua

{
    'abhinandh-s/age.nvim',
    cmd = { "Age" },
    config = function()
    local public_key = os.getenv("AGE_PUBLIC_KEY")
    local private_key = os.getenv("AGE_PRIVATE_KEY")

      require('age').setup({
        encrypt_and_del = true, -- will remove the original file after encrypting.
        public_key = public_key,
        private_key = private_key,
      })
    end
}
```

> [!TIP]
> check examples dir for non panic version

### config via lua file 

```lua
-- ~/.config/nvim/lua/plugins/age.lua

{
    'abhinandh-s/age.nvim',
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
```
```lua
-- ~/.config/nvim/lua/key.lua

return {
  private_key = "AGE-SECRET-KEY-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
}
```

## Usage

Age provides the `:Age` command with the following syntax:

```vim
:Age [action]
```

- `[action]` can be one of:
  - `encrypt`,
  - `decrypt`,
  - `genkey`

#### Examples:

- Generates an age key pair into key.txt in current working directory.

```vim
:Age genkey
```

- Kills the current buffer and switches to a previous buffer or creates a scratch buffer in case there is no buffer to switch, then encrypts the file with the provided age key.

```vim
:Age encrypt
```

- Decrypts the currently opened encrypted file, and switches to the decrypted file. 
```vim
:Age decrypt
```
## What is age?

[age](https://age-encryption.org/) is a simple, modern and secure file encryption tool.

It features small explicit keys, no config options, and UNIX-style composability.

### Why Choose Age Over GPG?

1. **Simplicity**: Age has a straightforward syntax and intuitive design, making it easier to use without extensive documentation.
2. **Modern Cryptography**: Age uses state-of-the-art cryptographic algorithms like X25519, ChaCha20-Poly1305, and HMAC-SHA256, ensuring robust security.
3. **Minimal Attack Surface**: Age's codebase is minimal and easier to audit compared to the complex and extensive GPG ecosystem.
4. **Portable Keys**: Age uses compact, user-friendly key formats, which are easy to manage and transfer.
5. **Focused Use Case**: Age is purpose-built for encrypting files securely and efficiently, without the additional complexity of key management and email encryption that GPG supports.


## License

Licensed under <a href="LICENSE">MIT license</a>.


Copyright (c) 2025 Abhinandh S

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
