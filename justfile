
default_package := "host"

build_guest:
    cargo component build --package guest --release && cp ./target/wasm32-wasip1/release/guest.wasm ./extensions/guest.wasm

run:
    just build_guest && cargo run --package host