{
  description = "MySK API Rust Dev Environment";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustTarget = pkgs.stdenv.hostPlatform.rust.rustcTarget;
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ rustTarget ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "mysk-rs";

          strictDeps = true;
          packages = with pkgs; [
            cargo-watch
            sqlx-cli
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
          ];

          shellHook = ''
            echo -e "Welcome to the \e[0;35mMy\e[0;36mSK\e[0m API Rust development environment!"
            export CARGO_BUILD_TARGET="${rustTarget}"
          '';
        };
      }
    );
}
