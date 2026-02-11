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

## Breaking Changes from v.2.2.0 [2026-02-11]

Due to security conserns the fields `private_key` and `public_key` in config has been removed. Instead user can specify `key_file`.

```diff 
  require('age').setup({
        encrypt_and_del = true, -- will remove the original file after encrypting.
-       private_key = "private_key"
-       public_key = "public_key"
+       key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
  })
```

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

> [!WARNING]
> This utility can be required by other plugins as a dependency to load a secret from your dotfiles to pass to a plugin.
>
> For example, plugin that requires an API key. For those people who keep their dotfiles public but want to load an API key for its setup.
> 
> I take no responsibility for leaked passwords or API keys. It is on you to decide what age is, this approach, and making sure to not commit your identity keys.
> 
> Given below is the safest config implemention. Never hardcode private key.

```lua 
 config = function()
  require('age').setup({
    encrypt_and_del = true,
    key_file = vim.fn.expand("~/.config/sops/age/keys.txt"), -- no need to specify `private_key` now
  })
 end
```

### Dependencies

- **None**
> [!NOTE]
> `age.nvim` does not require the `age` CLI to be installed separately.
Everything needed for encryption and decryption is handled within the plugin itself.


### config 

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
        key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
      })
    end
}
```

## Usage

Age provides:

- command - `:Age` 
- apis - `decrypt_to_string`, `decrypt_from_string` and `decrypt_to_string_with_identities`

The `:Age` command with the following syntax:

```vim
:Age [action]
```

- `[action]` can be one of:
  - `encrypt`,
  - `decrypt`,
  - `genkey`

#### Example usage of command:

- Generates an age key pair into key.txt in current working directory.

```vim
:Age genkey
```

- Kills the current buffer and switches to a previous buffer or creates a scratch buffer in case there is no buffer to switch, then encrypts the file with the provided age key.

```vim
:Age encrypt " uses key file from config
:Age encrypt /path/to/keys.txt " list for public keys
```

- Decrypts the currently opened encrypted file, and switches to the decrypted file. 
```vim
:Age decrypt
```

#### Example usage of api:

You can use age api in nvim configs as:

age.nvim provides 3 apis - 

- `decrypt_to_string` -- this uses key file provided in setup config 
- `decrypt_to_string_with_identities` -- takes from file
- `decrypt_from_string` -- takes from key file 

```lua 
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
```

```lua 
  -- trim whitespace if using for API tokens
  local secret = age.decrypt_to_string(path):gsub("%s+", "")
```

> [!NOTE]
> If you have any suggestions, please let me know.

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
