{
  description = "MySK API Rust Dev Environment";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.git-hooks.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          config,
          inputs',
          pkgs,
          ...
        }:
        let
          rustToolchain = inputs'.fenix.packages.stable;
        in
        {
          devShells.default = pkgs.mkShell {
            name = "mysk-rs";

            inputsFrom = [ config.pre-commit.devShell ];

            packages = with pkgs; [
              cargo-watch
              sqlx-cli
              pkg-config

              (rustToolchain.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
                "rust-analyzer"
              ])
            ];

            shellHook = ''
              echo -e "Welcome to the \e[0;35mMy\e[0;36mSK\e[0m API Rust development environment!"
              export CARGO_BUILD_TARGET="${pkgs.stdenv.hostPlatform.rust.rustcTarget}"
            '';
          };

          pre-commit = {
            check.enable = true;
            settings.hooks = {
              clippy = {
                enable = true;
                settings.extraArgs = "--no-deps";
                packageOverrides.cargo = rustToolchain.cargo;
                packageOverrides.clippy = rustToolchain.clippy;
              };
              rustfmt = {
                enable = true;
                packageOverrides.rustfmt = rustToolchain.rustfmt;
              };
            };
          };
        };
    };
}
