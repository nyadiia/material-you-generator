# material you theme generator
## A simple cli tool to take in an image and generate a material 3 theme from it.

Currently this generates all possible themes given a variant, which is light, light medium contrast, light high contrast, dark, dark medium contrast, and dark high contrast.
As of now the only 100% accurate generation is the light and dark theme, which matches a generation from [the website](https://material-foundation.github.io/material-theme-builder), however it is very close to the others.

# Build and run
To use this on your own system, clone this repo and then run
```sh
cargo build --release
```
and then run the resulting binary in target/release/

# TODO
- [ ] Parity
  - [ ] Schemes
    - [X] Light
    - [X] Dark
    - [ ] Light Medium Contrast
    - [ ] Dark Medium Contrast
    - [ ] Light High Contrast
    - [ ] Dark High Contrast
  - [ ] Palettes
- [ ] CLI Options
  - [X] Variant [default: tonal spot] \(monochrome, neutral, tonal-spot, vibrant, expressive, content, rainbow, or fruit-salad]
  - [X] Color Match
  - [ ] Polarity [default: all] \(light and/or dark)
  - [ ] Contrast [default: all] \(standard, medium, and/or high)
  - [ ] Output [default: theme.json]
    - [X] JSON
    - [ ] \(maybe) nix
- [ ] Nix integration
