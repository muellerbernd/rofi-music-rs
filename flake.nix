{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.outputs.legacyPackages.${system};
    in {
      packages.rofi_music_rs = pkgs.callPackage ./rofi-music.nix {
        inherit (pkgs.darwin.apple_sdk.frameworks) Security SystemConfiguration AppKit;
      };
      packages.default = self.outputs.packages.${system}.rofi_music_rs;

      devShells.default = self.packages.${system}.default.overrideAttrs (super: {
        nativeBuildInputs = with pkgs;
          super.nativeBuildInputs
          ++ [
            cargo
            clippy
            rustfmt
            rust-analyzer
          ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      });
    })
    // {
      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) rofi-music-rs;
      };
    };
}
