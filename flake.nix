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

        rustMsrv = pkgs.rust-bin.stable."1.70.0".minimal;
        rustLatest = pkgs.rust-bin.stable.latest.default;
        rustBeta = pkgs.rust-bin.beta.latest.minimal;
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

            rustLatest
            rustMsrv
            rustBeta

            (pkgs.writeShellScriptBin "cargo-msrv"
              ''exec ${rustMsrv}/bin/cargo "$@"'')
            (pkgs.writeShellScriptBin "cargo-latest"
              ''exec ${rustLatest}/bin/cargo "$@"'')
            (pkgs.writeShellScriptBin "cargo-beta"
              ''exec ${rustBeta}/bin/cargo "$@"'')
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
