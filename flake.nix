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

        registriesConf = pkgs.writeText "registries.conf" ''
          [registries.search]
          registries = ['docker.io']

          [registries.block]
          registries = []
        '';

        podmanSetupScript = pkgs.writeScript "podman-setup" ''
          #!${pkgs.runtimeShell}
          if ! test -f ~/.config/containers/policy.json; then
            install -Dm555 ${pkgs.skopeo.src}/default-policy.json ~/.config/containers/policy.json
          fi
          if ! test -f ~/.config/containers/registries.conf; then
            install -Dm555 ${registriesConf} ~/.config/containers/registries.conf
          fi
        '';

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
              sqlx-cli
              playwright-driver
              playwright-driver.browsers
              markdownlint-cli2
              podman
              skopeo
              slirp4netns
              fuse-overlayfs
            ]);

            shellHook = ''
              export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
              export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
              ${podmanSetupScript}
            '';
          };
        };
      }
    );
}
