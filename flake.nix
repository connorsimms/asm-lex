{
  description = "Development environment for asm-tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        stableToolchain = pkgs.rust-bin.stable."1.70.0".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "rustfmt"
            "clippy"
          ];
        };

        nightlyToolchain = pkgs.rust-bin.nightly.latest.minimal.override {
          extensions = [ "clippy" ];
        };

        nightlyClippy = pkgs.writeShellScriptBin "cargo-nightly-clippy" ''
          exec ${nightlyToolchain}/bin/cargo-clippy "$@"
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            gnumake
            gcc
            clang
            llvmPackages.llvm
            cargo-watch
            cargo-fuzz

            stableToolchain
            nightlyClippy
          ];

          buildInputs = with pkgs; [
            libiconv
          ];

          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };
      }
    );
}
