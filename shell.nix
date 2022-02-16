{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
} }:

pkgs.mkShell {
  # nativeBuildInputs = with pkgs; [ rustc cargo gcc glibc ];
  buildInputs = with pkgs; [
    rust-bin.nightly."2022-02-15".default
    aspell
    aspellDicts.uk
    aspellDicts.en
    sqlite
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
