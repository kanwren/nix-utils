{
  description = "Tool providing miscellaneous personal nix utilities";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      {
        packages = {
          default = self.packages.${system}.nix-utils;
          nix-utils = import ./default.nix {
            inherit system;
            inherit (nixpkgs) lib;
            inherit naersk fenix;
          };
        };

        apps = {
          default = self.apps.${system}.nix-utils;
          nix-utils = flake-utils.lib.mkApp {
            drv = self.packages.${system}.nix-utils;
          };
        };
      });
}
