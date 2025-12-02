{
  description = "A very basic flake";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    ...
  }: let
    lib = nixpkgs.lib;
  in {
    devShells =
      lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (
        system: let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [fenix.overlays.default];
          };
        in {
          default = pkgs.mkShell rec {
            packages =
              [
                fenix.packages.${system}.default.toolchain
              ]
              ++ (with pkgs; [
                rust-analyzer
              ]);
            buildInputs = packages;
          };
        }
      );
  };
}

