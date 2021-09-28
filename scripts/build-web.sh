#!/bin/sh

./scripts/wasm-bindgen-macroquad.sh rocket_sim

# https://github.com/WebAssembly/wabt
wasm-strip docs/wbindgen/rocket_sim_bg.wasm

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
