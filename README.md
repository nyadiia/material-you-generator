# yalda

Yalda (/ˌjɑːl.də/)[^1] is a simple CLI tool which can generate [Material You] color schemes
from input images. Currently, it generates all possible themes (light/dark + contrasts) for
the given variant.

## Build and run

### Manual

Clone this repo and then run
```sh
cargo build --release
```
and then run the resulting binary in `./target/release/`.

### Nix

This repo includes a `flake.nix` which can be used to run the CLI.

(todo: example)

## To do
- [ ] CLI Options
  - [X] Variant [default: tonal spot] \(monochrome, neutral, tonal-spot, vibrant, expressive, content, rainbow, or fruit-salad)
  - [X] Color Match
  - [ ] Polarity [default: all] \(light and/or dark)
  - [ ] Contrast [default: all] \(standard, medium, and/or high)
  - [ ] Output [default: theme.json]
    - [X] JSON
    - [ ] \(maybe) nix
- [ ] Nix integration


[^1]: Named after [Yaldabaoth], creator of the _material_ realm.

[Yaldabaoth]: https://en.wikipedia.org/wiki/Yaldabaoth
[Material You]: https://m3.material.io/
[Material Theme Builder]: https://material-foundation.github.io/material-theme-builder/
