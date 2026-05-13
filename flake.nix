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
      self,
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

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "x86_64-unknown-linux-gnu" ];
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
            echo "Welcome to the MySK API Rust development environment!"
            export CARGO_BUILD_TARGET="x86_64-unknown-linux-gnu"
          '';
        };
      }
    );
}
