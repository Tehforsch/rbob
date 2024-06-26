# For now, I have rustup in my path anyways, so this is more an
# illustration how to do this in general.
{
  description = "My dev shells";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { inherit system overlays; };
  in {
    devShells.rust_nightly = with pkgs;
    mkShell.override { stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv; } {
      buildInputs = [
        pkg-config
        (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
        cmake
        mpi
        hdf5
        libclang
        mold
        clang
      ];

      shellHook = ''
        export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
      '';
    };
  });
}
