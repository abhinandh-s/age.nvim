{
  description = "A basic rust devShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
    unstable-nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    unstable-nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
          (final: prev: {
            unstable = import unstable-nixpkgs {
              inherit system;
              config.allowUnfree = true;
            };
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            # nativeBuildInputs is usually what you want -- tools you need to run
            nativeBuildInputs = with pkgs.buildPackages; [lua];
            buildInputs = [
              openssl
              pkg-config
              llvmPackages.bintools
              eza
              unstable.neovim
              unstable.lazygit
              fd
              unstable.rustup
              rust-bin.stable.latest.default
            ];

            shellHook = ''
              alias ls=eza
              alias find=fd
              export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
              export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
              echo "Environment ready!" | ${pkgs.lolcat}/bin/lolcat
            '';
          };
      }
    );
}
