# rofi-music-rs

## How to build

Two ways possible:

1. use cargo
2. use the provided nix flake via `nix build`

## How to install

### On NixOS

Use the provided nix overlay:

```nix
inputs = {
...
  rofi-music-rs.url = "github:muellerbernd/rofi-music-rs";
...
};

```

and then include `rofi-music-rs` via

```nix
rofi-music-rs = inputs.rofi-music-rs.packages."x86_64-linux".rofi_music_rs;
```

## How to use

### via CLI

- build with cargo `cargo build --release`

```bash
./target/release/rofi-music-rs
./target/release/rofi-music-rs continue # run in continuous mode
./target/release/rofi-music-rs help # show help
```

- or use nix

```bash
nix build # creates result folder
./result/bin/rofi-music-rs
./result/bin/rofi-music-rs continue # run in continuous mode
./result/bin/rofi-music-rs help # show help
```
