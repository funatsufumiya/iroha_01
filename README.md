250129 study "iroha" / bevy 0.15

![screenshot.png](screenshot.png)

## Run (locally)

```bash
$ cargo run
```

## Build WASM

```bash
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --target web --out-dir . --no-typescript target/wasm32-unknown-unknown/release/iroha_01.wasm
```