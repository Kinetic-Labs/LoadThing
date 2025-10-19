{
  description = "LoadThing dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        openssl = pkgs.openssl;
      in {
        devShells = {
          default = pkgs.mkShell {
            name = "load-thing-dev-shell";

            buildInputs = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer
              git
              openssl
            ];

            OPENSSL_DIR = openssl;
            OPENSSL_LIB_DIR = "${openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
            PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
          };
        };
      });
}

