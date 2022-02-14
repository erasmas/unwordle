{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc cargo gcc glibc ];
  buildInputs = with pkgs; [ rustfmt aspell aspellDicts.uk aspellDicts.en sqlite readline ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
