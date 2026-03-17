# Changelog

All notable changes to this project will be documented in this file.

## [2.2.0] - 2026-02-11

### Breaking Changes!
* **Security Update:** Removed `private_key` and `public_key` fields from the configuration to prevent sensitive data from being hardcoded in config files. 
* Users must now use the `key_file` parameter to point to their age keys.

**Migration Example:**
```diff 
  require('age').setup({
        encrypt_and_del = true, -- will remove the original file after encrypting.
-       private_key = "private_key"
-       public_key = "public_key"
+       key_file = vim.fn.expand("~/.config/sops/age/keys.txt"),
  })
```
