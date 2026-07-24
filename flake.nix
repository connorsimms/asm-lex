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
        rustLatest = pkgs.rust-bin.stable.latest.default.override { extensions = [ "llvm-tools" "rust-analyzer" "rust-src" ]; };
        rustBeta = pkgs.rust-bin.beta.latest.minimal;
        rustNightly = pkgs.rust-bin.beta.latest.minimal;
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
            cargo-llvm-cov
            cargo-mutants
            cargo-audit

            rustLatest
            rustMsrv
            rustBeta
            rustNightly

            (pkgs.writeShellScriptBin "cargoLatest"
              ''exec ${rustLatest}/bin/cargo "$@"'')
            (pkgs.writeShellScriptBin "cargoMsrv"
              ''exec ${rustMsrv}/bin/cargo "$@"'')
            (pkgs.writeShellScriptBin "cargoBeta"
              ''exec ${rustBeta}/bin/cargo "$@"'')
            (pkgs.writeShellScriptBin "cargoNightly"
              ''exec ${rustNightly}/bin/cargo "$@"'')
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
