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

        gnuCrossFor = triple: (import pkgs.path {
          localSystem = pkgs.system;
          crossSystem = { config = triple; };
        }).buildPackages;

        mkGnuCrossShell = triple:
          let bp = gnuCrossFor triple;
          in pkgs.mkShell { nativeBuildInputs = [ bp.gcc bp.binutils ]; };

        fixtureTriples = [
          "x86_64-unknown-linux-gnu"
          "aarch64-unknown-linux-gnu"
          "armv7l-unknown-linux-gnueabihf"
          "riscv64-unknown-linux-gnu"
        ];

        gnuToolchains = builtins.concatMap
          (t: let bp = gnuCrossFor t; in [ bp.gcc bp.binutils ])
          fixtureTriples;
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
              cargo-show-asm

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

          aarch64-linux = mkGnuCrossShell "aarch64-unknown-linux-gnu";
          aarch64-windows = mkGnuCrossShell "aarch64-w64-mingw32";
          aarch64-none = mkGnuCrossShell "aarch64-none-elf";

          x86_64-linux = mkGnuCrossShell "x86_64-unknown-linux-gnu";
          x86_64-windows = mkGnuCrossShell "x86_64-w64-mingw32";
          x86_64-none = mkGnuCrossShell "x86_64-elf";

          arm-linux-gnueabihf = mkGnuCrossShell "armv7l-unknown-linux-gnueabihf";
          arm-none = mkGnuCrossShell "arm-none-eabi";

          riscv64-linux = mkGnuCrossShell "riscv64-unknown-linux-gnu";
          riscv64-none = mkGnuCrossShell "riscv64-none-elf";

          llvm = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              llvmPackages_21.clang
              llvmPackages_21.llvm
            ];
          };

          test-fixtures = pkgs.mkShell {
            nativeBuildInputs = gnuToolchains ++ [
              pkgs.llvmPackages_21.clang-unwrapped
              pkgs.llvmPackages_21.llvm
              rustLatest
            ];
            hardeningDisable = [ "all" ];
          };
        };
      }
    );
}
