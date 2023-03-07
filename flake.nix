{
  description = "The Shogun Rollup";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
        url = "github:ipetkov/crane";
        inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    celewasm = {
      url = "github:qasaur/celewasm";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    celewasm,
    rust-overlay
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustWithWasmTargetDev = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in rec
      {
        packages = {
          contracts = celewasm.lib.${system}.buildContracts ./wasm/. ./wasm/contracts/.;
        };

        apps = {
          deploy = celewasm.lib.${system}.deployContract;
          init = celewasm.lib.${system}.initContract;
          execute = celewasm.lib.${system}.executeContract;
          query = celewasm.lib.${system}.queryContract;
        };

        devShells = {
            default = pkgs.mkShell {
              name = "shogun-shell";

              packages = [
                rustWithWasmTargetDev
                pkgs.rust-analyzer
                pkgs.protobuf
                pkgs.go
                pkgs.gopls
                pkgs.gotools
                pkgs.go-tools

		pkgs.jq

              ];

            };
        };
      }
    );
}
