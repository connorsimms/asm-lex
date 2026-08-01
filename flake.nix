{
  description = "Development environment for asm-parse";

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

        rustLatest = pkgs.rust-bin.stable.latest.default.override { extensions = [ "llvm-tools" "rust-analyzer" "rust-src" ]; };
        rustMsrv = pkgs.rust-bin.stable."1.70.0".minimal;
        rustBeta = pkgs.rust-bin.beta.latest.minimal;
        rustNightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);

        nightlyRustPlatform = pkgs.makeRustPlatform {
          cargo = rustNightly;
          rustc = rustNightly;
        };

        cargo-minimal-versions = nightlyRustPlatform.buildRustPackage rec {
          pname = "cargo-minimal-versions";
          version = "0.1.37";
          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-J1dA3tfTqiFKGdMfZwgXvAoPY8QcWrP1kkD+HTbMwPI=";
          };
          cargoHash = "sha256-J9eInyzbvVRz9SDEKaJoLCNe2zNym2t/unPh0CrZxzQ=";
          doCheck = false;
        };
      in
      {
        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo-llvm-cov
              cargo-mutants
              cargo-insta
              cargo-hack
              cargo-minimal-versions
              cargo-flamegraph

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

          cross = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkgsCross.gnu64.buildPackages.gcc
              pkgsCross.gnu64.buildPackages.binutils
              pkgsCross.aarch64-multiplatform.buildPackages.gcc
              pkgsCross.aarch64-multiplatform.buildPackages.binutils
              pkgsCross.armv7l-hf-multiplatform.buildPackages.gcc
              pkgsCross.armv7l-hf-multiplatform.buildPackages.binutils
              pkgsCross.riscv64.buildPackages.gcc
              pkgsCross.riscv64.buildPackages.binutils
              csmith
            ];
          };
        };
      }
    );
}
