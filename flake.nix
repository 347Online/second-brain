{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem =
        {
          system,
          pkgs,
          ...
        }:
        let
          rust-toolchain = pkgs.fenix.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
          ];

          second-brain = pkgs.callPackage ./package.nix {
            rustPlatform = pkgs.rustPlatform;
            lib = pkgs.lib;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.fenix.overlays.default
            ];
          };

          devShells.default = pkgs.mkShell {
            inherit (second-brain) buildInputs;
          };

          formatter = pkgs.nixfmt-tree;

          packages = {
            inherit second-brain;
            default = second-brain;
          };
        };
    };
}
