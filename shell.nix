with import <nixpkgs> { };

mkShell {
  buildInputs = [
    nodejs
    yarn
    cargo
    rustc
    rustfmt
    curl
    libiconv
  ];
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
