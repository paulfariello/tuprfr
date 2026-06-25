{
  description = "tuprfr — Tu preferes ? web game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        commonPackages = with pkgs; [
          rustToolchain
          rust-analyzer
          sqlite
        ];

      in {
        devShells = {
          default = pkgs.mkShell {
            name = "tuprfr";
            packages = commonPackages;
          };

          dev = pkgs.mkShell {
            name = "tuprfr-dev";
            packages = commonPackages ++ (with pkgs; [
              neovim
              claude-code
              git
              pre-commit
              nodejs
              playwright-driver
              markdownlint-cli2
            ]);
          };
        };
      }
    );
}
