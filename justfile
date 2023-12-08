wasm_release:
    cargo build --target wasm32-wasi -p demo_plugin

run_release *ARGS='target/wasm32-wasi/debug/demo_plugin.wasm': wasm_release
    cargo run -p antz -- {{ARGS}}

wasm:
    cargo build --target wasm32-wasi -p demo_plugin

run *ARGS='target/wasm32-wasi/debug/demo_plugin.wasm': wasm
    cargo run -p antz -- {{ARGS}}

default:
  @just --list
