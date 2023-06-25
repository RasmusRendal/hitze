{
  description = "An optimizing brainfuck JIT interpreter";

  inputs = {
    nixpkgs.url = "nixpkgs/master";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      defaultPackage = pkgs.stdenv.mkDerivation {
        pname = "hitze";
        version = "0.1";
        buildInputs = with pkgs; [cargo rustc git rustfmt clippy gnuplot];
      };
      formatter = pkgs.alejandra;
    });
}
