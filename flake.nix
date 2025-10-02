{
  description = "What if correct horse battery staple, but Pokémon";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix.url = "github:nix-community/crate2nix";

    # Development

    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
    allow-import-from-derivation = true;
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      rust-overlay,
      crate2nix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      imports = [
        ./nix/rust-overlay/flake-module.nix
        ./nix/devshell/flake-module.nix
      ];

      perSystem =
        {
          system,
          pkgs,
          lib,
          ...
        }:
        let
          # If you dislike IFD, you can also generate it with `crate2nix generate`
          # on each dependency change and import it here with `import ./Cargo.nix`.
          cargoNix = crate2nix.tools.${system}.appliedCargoNix {
            name = "pkpw";
            src = builtins.path {
              path = ./.;
              name = "source";
            };
          };

          pkpw = cargoNix.rootCrate.build.override {
            crateOverrides = {
              pkpw = attrs: {
                buildInputs = lib.optionals pkgs.stdenv.isDarwin (
                  with pkgs.darwin.apple_sdk.frameworks;
                  [
                    AppKit
                  ]
                );
              };
            };
          };
        in
        {
          checks = {
            rustnix = pkpw.override {
              runTests = true;
            };
          };

          packages = {
            default = pkpw;

            inherit (pkgs) rust-toolchain;

            rust-toolchain-versions = pkgs.writeScriptBin "rust-toolchain-versions" ''
              ${pkgs.rust-toolchain}/bin/cargo --version
              ${pkgs.rust-toolchain}/bin/rustc --version
            '';
          };
        };
    };
}
