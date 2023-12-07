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
