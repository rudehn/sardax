# Getting Set Up
## Installing Rust
https://www.rust-lang.org/tools/install

## Releasing with WASM for Online Play
Following instructions from [here](https://bfnightly.bracketproductions.com/rustbook/webbuild.html)
 or [here](https://johanhelsing.studio/posts/extreme-bevy)

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install wasm-server-runner
cargo install cargo-watch
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\release\rust-roguelike.wasm --out-dir wasm --no-modules --no-typescript
```

## Running WASM build on Local Server

```
python3 -m http.server
# Navigate to 127.0.0.1:8000
```

# Additional Documentation
* [design-ideas](./docs/design-ideas.md)
* [known-bugs](./docs/known-bugs.md)
* [feature-requests](./docs/feature-requests.md)
