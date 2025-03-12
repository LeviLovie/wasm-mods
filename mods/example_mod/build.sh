#!/bin/sh
set -e

cargo build --target wasm32-unknown-unknown --release || { echo "Cargo build failed!" >&2; exit 1; }
wasm-tools component new target/wasm32-unknown-unknown/release/example_mod.wasm -o mod.wasm || { echo "wasm-tools failed!" >&2; exit 1; }
