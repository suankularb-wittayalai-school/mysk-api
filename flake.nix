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
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      git-hooks,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        git-hooks.flakeModule
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
          toolchain = inputs'.fenix.packages.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-+xUr6Al1wZSqX8aEsq7DpuFOK4Ve/i+qTl+K5uWQ9GM=";
          };
        in
        {
          devShells.default = pkgs.mkShell {
            name = "mysk-rs";

            inputsFrom = [ config.pre-commit.devShell ];

            packages = with pkgs; [
              cargo-watch
              sqlx-cli
              pkg-config

              # NOTE: Consider using a dedicated `rust-toolchain.toml` file
              # https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
              toolchain
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
                packageOverrides.cargo = toolchain;
                packageOverrides.clippy = toolchain;
              };
              rustfmt = {
                enable = true;
                packageOverrides.rustfmt = toolchain;
              };
            };
          };
        };
    };
}
