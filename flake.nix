{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      nci,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.nci.flakeModule
        inputs.treefmt-nix.flakeModule
        ./nix/treefmt.nix
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      perSystem =
        {
          config,
          self',
          pkgs,
          ...
        }:
        let
          name = "durak";
        in
        {
          nci = {
            projects = {
              ${name} = {
                path = ./.;
              };
            };
            crates = {
              ${name} = {
                export = true;
              };
            };
            toolchainConfig = ./rust-toolchain.toml;
          };
          packages = {
            default = self'.packages."${name}-release";
          };
          devShells.default = config.nci.outputs."${name}".devShell.overrideAttrs (old: {
	    packages = [
	      pkgs.just
	    ];
          });
        };
      flake = { };
    };
}
