#!/bin/bash

set -x

BINARY_NAME="galaxy_viewer"

cargo build --bin $BINARY_NAME --release --target wasm32-unknown-unknown

wasm-bindgen \
--no-typescript \
--target web \
--out-dir ./www/ \
--out-name $BINARY_NAME \
"./target/wasm32-unknown-unknown/release/${BINARY_NAME}.wasm"
