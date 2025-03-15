# Wasm Mods
This project is an example of how to use [wasmtime](https://wasmtime.dev/) as a modding system for games written in Rust. I use a [wrapper around wasmtime](https://github.com/DouglasDwyer/wasm_component_layer) made by [DouglasDwyer](https://github.com/DouglasDwyer).

## Goals
- [x] Event system
- [x] Internal data
- [x] Callbacks
- [ ] Graphics
- [ ] Player control example

## Project Structure
* `src` - Game logic.
* `wit` - [WIT](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) API defenitions.
* `mods` - Crates with module logic compiled to WASM.
* `crates` - Some packages to simplify code.
  * `mod_macros` - A few macros to make mod creation easier.
  * `mod_manager` - Abstraction for interacting with WASM packages.

## Running
Build occurs in two stages: main executable and mods. Mods are built through the `build.rs` file which runs build scripts inside mod directories and copies binaries into the `wasm` folder next to the executable.

> **_NOTE:_** The build scripts are in [bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) format. Support for windows is [planned in the future](https://github.com/LeviLovie/wasm-mods/issues/3)
 
Make sure you have [rust](https://www.rust-lang.org/tools/install), [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), and [wasm-tools](https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#installation) installed on your machine.

Install `wasm32-unknown-unknown` build target with:
```shell
rustup target add wasm32-unknown-unknown
```

Run with:
```shell
RUST_LOG=info cargo run
```
