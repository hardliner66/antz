wasm_release:
    cargo build --target wasm32-unknown-unknown -p demo_plugin --release

run_release *ARGS='target/wasm32-unknown-unknown/release/demo_plugin.wasm': wasm_release
    cargo run -p antz -- {{ARGS}}

wasm:
    cargo build --target wasm32-unknown-unknown -p demo_plugin

run *ARGS='target/wasm32-unknown-unknown/debug/demo_plugin.wasm': wasm
    cargo run -p antz -- {{ARGS}}

default:
  @just --list
