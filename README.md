# galaxy-explorer

Chase each other in a galaxy view, powered by the [`bevy` game engine][bevy].

🚧 WIP

## Useful links

- [bevy queries](https://taintedcoders.com/bevy/queries/)
- [bevy cheat book - guided tutorial](https://bevy-cheatbook.github.io/tutorial/guide.html)
- [bevy cheat book - pan/orbit camera](https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html)
- [bevy mod picking](https://github.com/aevyrie/bevy_mod_picking)
- [Echo server `tokio-tungstenite`](https://github.com/snapview/tokio-tungstenite/blob/master/examples/echo-server.rs)

[bevy]: https://bevyengine.org/

## Building for wasm

Setup

```bash
rustup target install wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

Build

```bash
cargo build --bin viewer_sp --release --target wasm32-unknown-unknown

wasm-bindgen \
            --no-typescript \
            --target web \
            --out-dir ./www/ \
            --out-name "viewer_sp" \
            ./target/wasm32-unknown-unknown/release/viewer_sp.wasm
```

Serve

```bash
python3 -m http.server --bind 127.0.0.1 --directory www 8080
```

## Contributing

All contributions are assumed to be under our [MIT license](./LICENSE).
