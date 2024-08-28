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
}
