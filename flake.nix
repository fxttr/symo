{
  description = "symo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        fenix-channel = fenix.packages.${system}.latest;

        fenix-toolchain = (fenix-channel.withComponents [
          "rustc"
          "cargo"
          "clippy"
          "rust-analysis"
          "rust-src"
          "rustfmt"
          "llvm-tools-preview"
        ]);

        craneLib = (crane.mkLib pkgs).overrideToolchain fenix-toolchain;

        hadron = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          doCheck = false;

          buildInputs = [ ];
        };
      in
      {
        checks = {
          inherit hadron;
        };

        packages.default = hadron;

        devShells.default = pkgs.mkShell {
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
          inputsFrom = builtins.attrValues self.checks;

          nativeBuildInputs = with pkgs; [
            fenix-toolchain
            rust-analyzer
            rustfmt
            clippy
            pkg-config
            zfs
            xorg.libX11
            pipewire
          ];
        };
      });
}
