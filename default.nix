{ system
, lib
, naersk
, fenix
}:

let
  toolchain = with fenix.packages.${system};
    combine [
      minimal.rustc
      minimal.cargo
    ];
  naersk-lib = naersk.lib.${system}.override {
    cargo = toolchain;
    rustc = toolchain;
  };
in
naersk-lib.buildPackage {
  src = ./.;
  pname = "nix-utils";
  meta = with lib; {
    description = "Tool providing miscellaneous personal nix utilities";
    platforms = platforms.unix;
  };
}
