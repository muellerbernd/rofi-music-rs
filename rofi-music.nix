# Helpful documentation: https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
{ lib, stdenv, installShellFiles, rustPlatform, libiconv, Security
, SystemConfiguration, AppKit, }:
rustPlatform.buildRustPackage {
  name = "rofi-music-rs";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    # Allow dependencies to be fetched from git and avoid having to set the outputHashes manually
    allowBuiltinFetchGit = true;
  };

  nativeBuildInputs = [ installShellFiles ];

  buildInputs = lib.optionals stdenv.isDarwin [
    libiconv
    Security
    SystemConfiguration
    AppKit
  ];

  # postInstall = ''
  #   installShellCompletion --cmd atuin \
  #     --bash <($out/bin/atuin gen-completions -s bash) \
  #     --fish <($out/bin/atuin gen-completions -s fish) \
  #     --zsh <($out/bin/atuin gen-completions -s zsh)
  # '';

  # Additional flags passed to the cargo test binary, see `cargo test -- --help`
  # checkFlags = [
  #   # Sync tests require a postgres server
  #   "--skip=sync"
  #   "--skip=registration"
  # ];

  meta = with lib; {
    description = "rofi-music-rs";
    homepage = "";
    license = licenses.mit;
  };
}
