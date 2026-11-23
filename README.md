# Replay Studio

The replay editor for [Horizon](https://github.com/horizon-rust/horizon), as a sandboxed WebAssembly mod
(`kind = "mod"`). Recording, `.mcpr` files and playback stay in the game; this mod drives them through the replay
calls of `horizon-mod-api` and draws the editor: recordings browser, timeline, free camera and camera paths.

Work in progress: this first version only registers its key (F8 by default). The editor arrives in steps, tracked in
[horizon-rust/horizon#15406](https://github.com/horizon-rust/horizon/issues/15406) and
[#18628](https://github.com/horizon-rust/horizon/issues/18628).

## Build

Builds run in CI (`.github/workflows/ci.yml`): format, clippy, tests and the wasm `.hmod`, kept as an artifact.
Locally, with `cargo-horizon-mod` installed from the pinned SDK build in `Cargo.toml`:

```sh
cargo horizon-mod check
cargo horizon-mod build --wasm
```

## Licence

MIT.
