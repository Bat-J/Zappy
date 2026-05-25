{ pkgs ? import <nixpkgs> {}, mcc-env }:

let
  mccShell = pkgs.mkShell.override { stdenv = mcc-env; };
in
mccShell {
  name = "zapfrites";
  buildInputs = with pkgs; [
    clang-tools
    cmake
    gnumake
    # Rust toolchain
    rustup
    rustc
    cargo
    rustfmt
    rust-analyzer
  ];

  shellHook = ''
    echo "Entering Nix shell: C++ + Rust (cargo) environment"
  '';
}
