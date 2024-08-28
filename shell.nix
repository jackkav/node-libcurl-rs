with import <nixpkgs> { };

mkShell {
  buildInputs = [
    nodejs
    yarn
    cargo
    rustc
  ];
}
