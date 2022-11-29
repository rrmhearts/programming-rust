
Project was instantiated with `cargo generate --git https://github.com/rustwasm/wasm-pack-template`.

The `src/lib.rs` file is the root of the Rust crate that we are compiling to WebAssembly. It uses `wasm-bindgen` to interface with JavaScript. It imports the window.alert JavaScript function, and exports the greet Rust function, which alerts a greeting message.

The `src/utils.rs` module provides common utilities to make working with Rust compiled to WebAssembly easier. We will take a look at some of these utilities in more detail later in the tutorial, such as when we look at debugging our wasm code, but we can ignore this file for now.

We use `wasm-pack` to orchestrate the following build steps:

* Ensure that we have Rust 1.30 or newer and the wasm32-unknown-unknown target installed via rustup,
* Compile our Rust sources into a WebAssembly .wasm binary via cargo,
* Use wasm-bindgen to generate the JavaScript API for using our Rust-generated WebAssembly.
To do all of that, run this command inside the project directory:
```
wasm-pack build
```